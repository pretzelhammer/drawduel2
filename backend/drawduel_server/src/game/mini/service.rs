#![allow(unused_variables, dead_code)]

use axum::extract::connect_info::ConnectInfo;
use axum::{
    body::Bytes,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
};
use drawduel_engine::game::mini::*;
use hyper::StatusCode;
use prost::Message as ProstMessage;
use serde::Deserialize;
use std::net::SocketAddr;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    sync::{broadcast, mpsc, oneshot},
    time::{self},
};

type PlayerId = u32;
type SerializedMsg = Bytes;
type UniqueSerializedMsg = Vec<u8>;
type GameTx = broadcast::Sender<SerializedMsg>;
type GameRx = broadcast::Receiver<SerializedMsg>;
type RoomTx = mpsc::Sender<RoomEvent>;
type RoomRx = mpsc::Receiver<RoomEvent>;
type RegisterTx =
    oneshot::Sender<Result<(PlayerId, UniqueSerializedMsg, GameRx), SeError>>;
type RegisterRx =
    oneshot::Receiver<Result<(PlayerId, UniqueSerializedMsg, GameRx), SeError>>;

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

#[derive(Debug)]
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
    fn reset(&mut self) {
        self.passes.clear();
        self.disconnects.clear();
    }
}

#[derive(Debug)]
enum RoomEvent {
    ClientConnect {
        register_tx: RegisterTx,
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

impl Display for RoomEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let RoomEvent::ClientConnect {
            register_tx,
            client_info,
        } = self
        {
            f.debug_struct("ClientConnect")
                .field("register_tx", &"register_tx")
                .field("client_info", client_info)
                .finish()
        } else {
            Debug::fmt(self, f)
        }
    }
}

fn error_into_response(
    error: SeError,
    client_info: ClientInfo,
) -> axum::response::Response {
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
    tracing::trace!(
        "{:?} w/pass {} @ {addr} connecting",
        client_info.name,
        client_info.pass
    );
    let room_tx = shared_service_state.room_tx;

    // 1. player has to register themself with room
    let (register_tx, register_rx) = oneshot::channel();
    if let Err(err) = room_tx
        .send(RoomEvent::ClientConnect {
            register_tx: register_tx,
            client_info: client_info.clone(),
        })
        .await
    {
        tracing::error!(
            "MINI GAME ROOM SHOULD ALWAYS EXIST AND ACCEPT ALL MSGS: {err}"
        );
    }
    let recv_result = register_rx.await;
    if let Ok(register_result) = recv_result {
        match register_result {
            Ok((player_id, set_game_event, game_rx)) => {
                ws.on_upgrade(move |socket| {
                    player_manager(
                        socket,
                        addr,
                        player_id,
                        set_game_event,
                        game_rx,
                        (*room_tx).clone(),
                    )
                })
            }
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
    events.clear();
    (game, events, serialized)
}

fn serialize_server_events(
    mut events: Vec<ServerEvent>,
) -> (Vec<ServerEvent>, SerializedMsg) {
    let server_events = ServerEvents { events: events };
    let mut serialized = Vec::<u8>::with_capacity(32);
    server_events
        .encode(&mut serialized)
        .expect("serialized server events");
    events = server_events.events;
    events.clear();
    (events, Bytes::from(serialized))
}

async fn room_manager(game_tx: GameTx, mut room_rx: RoomRx) {
    let mut next_player_id = 0u32;
    let mut game = Game::new();
    let mut room_state = RoomState::new();
    let mut events: Vec<ServerEvent> = Vec::with_capacity(4);
    loop {
        let event = match room_rx.recv().await {
            None => {
                tracing::error!("DESTROYING ONLY MINIGAME ROOM NOOOOO!!!");
                break;
            }
            Some(event) => event,
        };

        tracing::trace!("mini game state {game:?}");
        tracing::trace!("room got event {event}");
        match event {
            RoomEvent::ClientConnect {
                register_tx,
                client_info,
            } => {
                let ClientInfo { name, pass } = client_info;

                // check if this an existing player reconnecting
                if let Some(&player_id) = room_state.passes.get(&pass) {
                    // let other players know this player has reconnected
                    let player_connect = ServerEvent {
                        se_type: Some(SeType::PlayerConnect(SePlayerConnect {
                            id: player_id,
                        })),
                    };

                    // not sure when this would ever be false, maybe if player
                    // disconnected due to stale connection earlier?
                    game.advance(player_connect, &mut events);
                    let player_connected = !events.is_empty();
                    debug_assert!(
                        player_connected,
                        "player {player_id} connected to room but was already connected in game"
                    );
                    if player_connected {
                        let (reused_events, serialized_msg) =
                            serialize_server_events(events);
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
                    register_tx
                        .send(Ok((
                            player_id,
                            serialized_msg,
                            game_tx.subscribe(),
                        )))
                        .expect("sent init msg to reconnecting player");

                // otherwise this is a new player connecting
                } else {
                    // let other players know this player has joined
                    let new_player_id = next_player_id;
                    next_player_id += 1;
                    let player_name = name
                        .unwrap_or_else(|| format!("player{new_player_id:02}"));
                    let player_join = ServerEvent {
                        se_type: Some(SeType::PlayerJoin(SePlayerJoin {
                            id: new_player_id,
                            name: player_name,
                        })),
                    };

                    // not sure when this would ever be false, something very wrong
                    // must have occurred for this to somehow be false
                    game.advance(player_join, &mut events);
                    let player_joined = !events.is_empty();
                    debug_assert!(player_joined, "new player {new_player_id} connected but was already present in game state");
                    if player_joined {
                        room_state.passes.insert(pass, new_player_id);

                        let multiple_players = game.connected_players() > 1;
                        tracing::trace!("multiple players {multiple_players}");

                        // only send msg if there are other players
                        // to receive it
                        if multiple_players {
                            let (reused_events, serialized_msg) =
                                serialize_server_events(events);
                            events = reused_events;
                            if let Err(err) = game_tx.send(serialized_msg) {
                                tracing::error!(
                                    "sent game message to empty game: {err}"
                                );
                            }
                        } else {
                            events.clear();
                        }
                    }

                    // sent joined player current game state
                    let (reused_game, reused_events, serialized_msg) =
                        serialize_set_game(game, events, new_player_id);
                    game = reused_game;
                    events = reused_events;
                    register_tx
                        .send(Ok((
                            new_player_id,
                            serialized_msg,
                            game_tx.subscribe(),
                        )))
                        .expect("sent init msg to reconnecting player");
                }
            }
            RoomEvent::ClientEvent {
                player_id,
                client_event,
            } => {
                let server_event =
                    ServerEvent::from_client(player_id, client_event);
                game.advance(server_event, &mut events);
                let advanced = !events.is_empty();
                if advanced {
                    let (reused_events, serialized_msg) =
                        serialize_server_events(events);
                    events = reused_events;
                    game_tx
                        .send(serialized_msg)
                        .expect("sent server event to all players");
                }
            }
            RoomEvent::ClientDisconnect { player_id } => {
                let server_event = ServerEvent {
                    se_type: Some(SeType::PlayerDisconnect(
                        SePlayerDisconnect { id: player_id },
                    )),
                };
                game.advance(server_event, &mut events);
                let advanced = !events.is_empty();
                if advanced {
                    let (reused_events, serialized_msg) =
                        serialize_server_events(events);
                    events = reused_events;
                    if let Err(err) = game_tx.send(serialized_msg) {
                        // if we're here it means all players have disconnected
                        game.reset();
                        room_state.reset();
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
    mut game_rx: GameRx,
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
                // tracing::trace!("alive check for {player_id}, now {now:?}, last_client_event {last_client_event:?}, diff {diff:?}");
                if diff > stale_duration {
                    tracing::trace!("dropping stale player {player_id} @ {addr}");
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
                        tracing::trace!("failed to send ping to player {player_id} @ {addr}: {err}");
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
                                        tracing::warn!("failed to decode player event: {err}");
                                    },
                                }
                            },
                            Message::Close(close) => {
                                tracing::trace!("got close from {player_id}: {close:?}");
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
                                tracing::trace!("got ping from {player_id}: {ping:?}");
                                // no-op, axum auto-pongs for us
                            },
                            Message::Pong(pong) => {
                                tracing::trace!("got pong from {player_id}: {pong:?}");
                                // no-op
                            },
                            Message::Text(text) => {
                                tracing::trace!("got pong from {player_id}: {text}");
                                // no-op
                            },
                        }
                    } else {
                        tracing::trace!("player {player_id} @ {addr} disconnected abruptly: {}", result_msg.unwrap_err());
                        room_tx
                            .send(RoomEvent::ClientDisconnect {
                                player_id,
                            })
                            .await
                            .expect("sent disconnect msg");
                        break;
                    }
                } else {
                    tracing::trace!("player {player_id} @ {addr} closed connection");
                    room_tx
                        .send(RoomEvent::ClientDisconnect {
                            player_id,
                        })
                        .await
                        .expect("sent disconnect msg");
                    break;
                }
            },
            recv_result = game_rx.recv() => {
                if let Ok(serialized_msg) = recv_result {
                    let send_result = socket.send(Message::Binary(serialized_msg)).await;
                    if let Err(_) = send_result {
                        tracing::trace!("player {player_id} @ {addr} failed to send game event, breaking");
                        room_tx
                            .send(RoomEvent::ClientDisconnect {
                                player_id,
                            })
                            .await
                            .expect("sent disconnect msg");
                        break;
                    }
                } else {
                    tracing::trace!("player {player_id} @ {addr} failed to recv game event, breaking");
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
