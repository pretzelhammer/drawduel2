use crate::game::mini::engine::*;

use axum::{
    body::Bytes,
    extract::{
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
    routing::any,
    Router,
};
use axum_extra::TypedHeader;
use dashmap::DashMap;
use hyper::{client::conn, StatusCode};
use prost::Message as ProstMessage;
use serde::Deserialize;
use tokio::{
    net::unix::pipe::Receiver,
    sync::{broadcast, mpsc, oneshot},
    time::{self, sleep},
};

use std::{
    collections::HashMap,
    ops::ControlFlow,
    sync::Arc,
    time::{Duration, Instant},
};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

// allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

type PlayerId = u32;
type SerializedMsg = Arc<Vec<u8>>;
type UniqueSerializedMsg = Vec<u8>;
type GameTx = broadcast::Sender<SerializedMsg>;
type GameRx = broadcast::Receiver<SerializedMsg>;
type RoomTx = mpsc::Sender<RoomEvent>;
type RoomRx = mpsc::Receiver<RoomEvent>;
type RegisterTx = oneshot::Sender<Result<(PlayerId, UniqueSerializedMsg, GameRx), SeError>>;
type RegisterRx = oneshot::Receiver<Result<(PlayerId, UniqueSerializedMsg, GameRx), SeError>>;

#[derive(Deserialize, Clone, Debug)]
pub struct ClientInfo {
    name: Option<String>,
    pass: String,
}

#[derive(Clone)]
pub struct SharedServiceState {
    room_tx: Arc<RoomTx>,
}

const CHANNEL_CAPACITY: usize = 2048;

impl SharedServiceState {
    pub fn new() -> Self {
        let (room_tx, room_rx) = mpsc::channel(CHANNEL_CAPACITY);
        let (game_tx, _) = broadcast::channel(CHANNEL_CAPACITY);
        let _ = tokio::spawn(room_manager(game_tx, room_rx));
        SharedServiceState {
            room_tx: Arc::new(room_tx),
        }
    }
}

struct RoomState {
    passes: HashMap<String, u32>,
    disconnects: Vec<(PlayerId, Instant)>,
}

impl RoomState {
    fn new() -> Self {
        Self {
            passes: HashMap::new(),
            disconnects: Vec::new(),
        }
    }
}

#[derive(Debug)]
enum RoomEvent {
    ClientConnect {
        finish_registration: RegisterTx,
        client_info: ClientInfo,
    },
    ClientDisconnect {
        player_id: PlayerId,
    },
    ClientEvent {
        player_id: u32,
        client_event: ClientEvent,
    },
}

fn error_into_response(error: SeError, client_info: ClientInfo) -> axum::response::Response {
    match SeErrorType::try_from(error.se_error_type) {
        Ok(SeErrorType::AlreadyConnected) => (
            StatusCode::CONFLICT,
            format!(
                "{:?} w/pass {} already connected to mini game",
                client_info.name, client_info.pass,
            ),
        )
            .into_response(),
        err => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "failed to connect {:?} w/pass {} to mini game",
                client_info.name, client_info.pass,
            ),
        )
            .into_response(),
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(client_info): Query<ClientInfo>,
    State(shared_service_state): State<SharedServiceState>,
) -> impl IntoResponse {
    println!(
        "{:?} w/pass {} at {addr} connected",
        client_info.name, client_info.pass
    );
    let room_tx = shared_service_state.room_tx;

    // 1. player has to register themself with room
    let (register_tx, register_rx) = oneshot::channel();
    room_tx
        .send(RoomEvent::ClientConnect {
            finish_registration: register_tx,
            client_info: client_info.clone(),
        })
        .await
        .unwrap(); // TODO: this panics if players leave and rejoin room
    let recv_result = register_rx.await;
    if let Ok(register_result) = recv_result {
        match register_result {
            Ok((player_id, set_game_event, game_rx)) => ws.on_upgrade(move |socket| {
                player_manager(
                    socket,
                    addr,
                    player_id,
                    set_game_event,
                    game_rx,
                    (*room_tx).clone(),
                )
            }),
            Err(error) => error_into_response(error, client_info),
        }
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "failed to register {:?} w/pass {} to mini game",
                client_info.name, client_info.pass,
            ),
        )
            .into_response()
    }
}

fn serialize_set_game(
    mut game: Game,
    mut events: Vec<ServerEvent>,
    player_id: u32,
) -> (Game, Vec<ServerEvent>, UniqueSerializedMsg) {
    events.push(ServerEvent {
        se_type: Some(SeType::SetGame(SeSetGame {
            player_id: player_id,
            game: Some(game),
        })),
    });
    let server_events = ServerEvents { events: events };
    let mut serialized = Vec::with_capacity(256);
    server_events
        .encode(&mut serialized)
        .expect("was able to encode set game state msg");
    // get reusable buffer back out
    events = server_events.events;
    // get reusable game back out
    game = {
        let server_event = events.pop().unwrap();
        if let Some(SeType::SetGame(server_game)) = server_event.se_type {
            server_game.game.unwrap()
        } else {
            panic!("this should be impossible")
        }
    };
    (game, events, serialized)
}

impl ServerEvent {
    fn from_client(player_id: PlayerId, client_event: ClientEvent) -> Self {
        match client_event.ce_type.unwrap() {
            CeType::Rename(rename) => ServerEvent {
                se_type: Some(SeType::PlayerRename(SePlayerRename {
                    id: player_id,
                    name: rename.name,
                })),
            },
            CeType::IncreaseScore(increase_score) => ServerEvent {
                se_type: Some(SeType::PlayerIncreaseScore(SePlayerIncreaseScore {
                    id: player_id,
                    score: increase_score.score,
                })),
            },
        }
    }
}

fn serialize_server_events(
    server_event: ServerEvent,
    mut events: Vec<ServerEvent>,
) -> (Vec<ServerEvent>, SerializedMsg) {
    events.push(server_event);
    let server_events = ServerEvents { events: events };
    let mut serialized = Vec::<u8>::with_capacity(32);
    server_events
        .encode(&mut serialized)
        .expect("serialized server events");
    events = server_events.events;
    events.clear();
    (events, Arc::new(serialized))
}

async fn room_manager(game_tx: GameTx, mut room_rx: RoomRx) {
    let mut next_player_id = 0u32;
    let mut game = Game::new();
    let mut room_state = RoomState::new();
    let mut events: Vec<ServerEvent> = Vec::with_capacity(4);
    loop {
        let event = room_rx.recv().await;
        if event.is_none() {
            println!("DESTROYING ONLY MINIGAME ROOM NOOOOO!!!");
            break;
        }
        println!("room got event {event:?}");
        match event.unwrap() {
            RoomEvent::ClientConnect {
                finish_registration,
                client_info,
            } => {
                let ClientInfo { name, pass } = client_info;

                // check if this an existing player reconnecting
                if let Some(&player_id) = room_state.passes.get(&pass) {
                    // let other players know this player has reconnected
                    let player_connect = ServerEvent {
                        se_type: Some(SeType::PlayerConnect(SePlayerConnect { id: player_id })),
                    };

                    // not sure when this would ever be false, maybe if player
                    // disconnected due to stale connection earlier?
                    let player_connected = game.advance(player_connect);
                    debug_assert!(
                        player_connected.is_some(),
                        "player {player_id} connected to room but was already connected in game"
                    );
                    if let Some(server_event) = player_connected {
                        let (reused_events, serialized_msg) =
                            serialize_server_events(server_event, events);
                        events = reused_events;
                        game_tx
                            .send(serialized_msg)
                            .expect("sent player connect msg");
                    }

                    // send connected player current game state
                    let (reused_game, reused_events, serialized_msg) =
                        serialize_set_game(game, events, player_id);
                    game = reused_game;
                    events = reused_events;
                    finish_registration
                        .send(Ok((player_id, serialized_msg, game_tx.subscribe())))
                        .expect("sent init msg to reconnecting player");

                // otherwise this is a new player connecting
                } else {
                    // let other players know this player has joined
                    let new_player_id = next_player_id;
                    next_player_id += 1;
                    let player_name = name.unwrap_or_else(|| format!("player{new_player_id:02}"));
                    let player_join = ServerEvent {
                        se_type: Some(SeType::PlayerJoin(SePlayerJoin {
                            id: new_player_id,
                            name: player_name,
                        })),
                    };
                    let is_very_first_player = new_player_id == 0;

                    // not sure when this would ever be false, something very wrong
                    // must have occurred for this to somehow be false
                    let player_joined = game.advance(player_join);
                    debug_assert!(player_joined.is_some(), "new player {new_player_id} connected but was already present in game state");
                    if let Some(server_event) = player_joined {
                        room_state.passes.insert(pass, new_player_id);

                        // no need to send this event if there are no other
                        // players yet, which is why we check if this is the
                        // first player
                        if !is_very_first_player {
                            let (reused_events, serialized_msg) =
                                serialize_server_events(server_event, events);
                            events = reused_events;
                            game_tx.send(serialized_msg).expect("sent player join msg");
                        }
                    }

                    // sent joined player current game state
                    let (reused_game, reused_events, serialized_msg) =
                        serialize_set_game(game, events, new_player_id);
                    game = reused_game;
                    events = reused_events;
                    finish_registration
                        .send(Ok((new_player_id, serialized_msg, game_tx.subscribe())))
                        .expect("sent init msg to reconnecting player");
                }
            }
            RoomEvent::ClientEvent {
                player_id,
                client_event,
            } => {
                let server_event = ServerEvent::from_client(player_id, client_event);
                let advanced = game.advance(server_event);
                if let Some(server_event) = advanced {
                    let (reused_events, serialized_msg) =
                        serialize_server_events(server_event, events);
                    events = reused_events;
                    game_tx
                        .send(serialized_msg)
                        .expect("sent server event to all players");
                }
            }
            RoomEvent::ClientDisconnect { player_id } => {
                let server_event = ServerEvent {
                    se_type: Some(SeType::PlayerDisconnect(SePlayerDisconnect {
                        id: player_id,
                    })),
                };
                let advanced = game.advance(server_event);
                if let Some(server_event) = advanced {
                    println!("mini game state {game:?}");
                    let (reused_events, serialized_msg) =
                        serialize_server_events(server_event, events);
                    events = reused_events;
                    if let Err(err) = game_tx.send(serialized_msg) {
                        // if we're here it means all players have disconnected
                        // no-op
                    }
                }
            }
        }
    }
}

async fn player_manager(
    mut socket: WebSocket,
    addr: SocketAddr,
    player_id: u32,
    set_game_event: UniqueSerializedMsg,
    game_rx: GameRx,
    room_tx: RoomTx,
) {
    let inactive_duration = Duration::from_millis(4000);
    let stale_duration = Duration::from_millis(9000);
    let alive_duration = Duration::from_millis(5000);
    let mut alive_interval = time::interval(alive_duration);
    let mut last_client_event = Instant::now();

    socket
        .send(Message::Binary(Bytes::from(set_game_event)))
        .await
        .expect("sent set game state to connected player");

    loop {
        tokio::select! {
            // note to self: double pings seem to be bad,
            // if sent a ping always wait for a pong, do
            // not send a 2nd ping, and if a pong doesn't
            // return after some timeout consider the connection
            // stale and can drop it without a close message,
            // since the client didn't respond to our ping
            // it's not like it would see the close message anyway
            _ = alive_interval.tick() => {
                let now = Instant::now();
                let diff = now.duration_since(last_client_event);
                // println!("alive check for {player_id}, now {now:?}, last_client_event {last_client_event:?}, diff {diff:?}");
                if diff > stale_duration {
                    println!("dropping stale player {player_id} @ {addr}");
                    room_tx
                        .send(RoomEvent::ClientDisconnect {
                            player_id,
                        })
                        .await
                        .expect("sent disconnect msg");
                    break;
                }
                if diff > inactive_duration {
                    let send_result = socket.send(Message::Ping(Bytes::from_static(b""))).await;
                    if let Err(err) = send_result {
                        println!("failed to send ping to player {player_id} @ {addr}: {err}");
                        room_tx
                            .send(RoomEvent::ClientDisconnect {
                                player_id,
                            })
                            .await
                            .expect("sent disconnect msg");
                        break;
                    }
                }
            },
            maybe_msg = socket.recv() => {
                last_client_event = Instant::now();
                if let Some(result_msg) = maybe_msg {
                    if let Ok(msg) = result_msg {
                        match msg {
                            Message::Binary(bytes) => {
                                match ClientEvent::decode(bytes) {
                                    Ok(client_event) => {
                                        room_tx
                                            .send(RoomEvent::ClientEvent {
                                                player_id,
                                                client_event,
                                            })
                                            .await
                                            .expect("sent player event to room");
                                    },
                                    Err(err) => {
                                        println!("failed to decode player event: {err}");
                                    },
                                }
                            },
                            Message::Close(_) => {
                                // axum handles response automatically
                                // but we still want to break out of
                                // this loop if player is quitting
                                room_tx
                                    .send(RoomEvent::ClientDisconnect {
                                        player_id,
                                    })
                                    .await
                                    .expect("sent disconnect msg");
                                break;
                            },
                            // no-op, we can ignore pings & pongs
                            // since axum handles those for us
                            // automatically, and we can ignore text
                            // messages since we only communicate
                            // by binary
                            Message::Ping(ping) => {
                                println!("got ping from {player_id}: {ping:?}");
                                // no-op, axum auto-pongs for us
                            },
                            Message::Pong(pong) => {
                                println!("got pong from {player_id}: {pong:?}");
                                // no-op
                            },
                            Message::Text(text) => {
                                println!("got pong from {player_id}: {text}");
                                // no-op
                            },
                        }
                    } else {
                        println!("{player_id} @ {addr} disconnected abruptly: {}", result_msg.unwrap_err());
                        room_tx
                            .send(RoomEvent::ClientDisconnect {
                                player_id,
                            })
                            .await
                            .expect("sent disconnect msg");
                        break;
                    }
                } else {
                    println!("{player_id} @ {addr} closed connection");
                    room_tx
                        .send(RoomEvent::ClientDisconnect {
                            player_id,
                        })
                        .await
                        .expect("sent disconnect msg");
                    break;
                }
            },
        }
    }
}
