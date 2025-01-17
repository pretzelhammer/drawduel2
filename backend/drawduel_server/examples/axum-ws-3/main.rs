#![allow(unused_imports, dead_code, unused_variables)]

use axum::{
    body::Bytes,
    extract::{ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade}, Query, State},
    response::IntoResponse,
    routing::any,
    Router,
};
use axum_extra::TypedHeader;
use dashmap::DashMap;
use hyper::{client::conn, StatusCode};
use prost::Message as ProstMessage;
use serde::Deserialize;
use tokio::{net::unix::pipe::Receiver, sync::{broadcast, mpsc, oneshot}, time::{self, sleep}};

use std::{collections::HashMap, ops::ControlFlow, sync::Arc, time::{Duration, Instant}};
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

mod generated_rust;
use generated_rust::{
    player_event, server_event, ErrorType, GameState, Player, PlayerEvent, ServerEvent, ServerEventError, ServerEventPlayerConnect, ServerEventPlayerDisconnect, ServerEventPlayerIncreaseScore, ServerEventPlayerJoin, ServerEventPlayerRename, ServerEventSetGameState, ServerEvents
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = Router::new()
        .route("/ws", any(ws_handler))
        .with_state(Rooms::new())
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

#[derive(Deserialize, Clone, Debug)]
struct ConnectQuery {
    name: String,
    pass: String,
    room: String,
}

#[derive(Clone)]
#[repr(transparent)]
struct Rooms(Arc<DashMap<String, Room>>);

impl Rooms {
    fn new() -> Self {
        Self(Arc::new(DashMap::with_capacity(8)))
    }
}

type PlayerId = u32;
type SerializedMsg = Arc<Vec<u8>>;
type UniqueSerializedMsg = Vec<u8>;
type GameTx = broadcast::Sender<SerializedMsg>;
type GameRx = broadcast::Receiver<SerializedMsg>;
type RoomTx = mpsc::Sender<RoomEvent>;
type RoomRx = mpsc::Receiver<RoomEvent>;
type RegisterTx = oneshot::Sender<Result<(PlayerId, UniqueSerializedMsg, GameRx), ServerEventError>>;
type RegisterRx = oneshot::Receiver<Result<(PlayerId, UniqueSerializedMsg, GameRx), ServerEventError>>;

struct Room {
    game_tx: GameTx,
    room_tx: RoomTx,
}

impl Room {
    fn new(
        game_tx: GameTx,
        room_tx: RoomTx
    ) -> Self {
        Self {
            game_tx,
            room_tx,
        }
    }
}

#[derive(Debug)]
enum RoomEvent {
    PlayerConnect {
        respond: RegisterTx,
        connect: ConnectQuery,
    },
    PlayerDisconnect {
        player_id: PlayerId,
    },
    PlayerEvent {
        player_id: u32,
        player_event: PlayerEvent,
    },
}

fn error_into_response(
    error: ServerEventError,
    query: ConnectQuery,
) -> axum::response::Response {
    match ErrorType::try_from(error.r#type) {
        Ok(ErrorType::AlreadyConnected) => {
            (
                StatusCode::CONFLICT,
                format!(
                    "{} w/pass {} already connected to room {}",
                    query.name,
                    query.pass,
                    query.room,
                )
            ).into_response()
        },
        err => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "failed to connect {} w/pass {} to room {}",
                    query.name,
                    query.pass,
                    query.room,
                )
            ).into_response()
        }
    }
}

impl GameState {
    fn new() -> Self {
        Self {
            players: HashMap::new(),
            admin: 0,
        }
    }
    // true if no players, or all players disconnected
    fn empty(&self) -> bool {
        let connected_players = self.players
            .iter()
            .fold(0, |acc, (_, p)| if p.connected { acc + 1 } else { acc });
        connected_players == 0
    }
    fn advance(&mut self, event: &ServerEvent) -> bool {
        use server_event::Type::*;
        match event.r#type.as_ref().unwrap() {
            PlayerJoin(player_join) => {
                let overwrote_existing_player = self.players.insert(
                    player_join.id,
                    Player {
                        name: player_join.name.clone(),
                        score: 0,
                        connected: true,
                        ready: false,
                    }
                ).is_some();
                if overwrote_existing_player {
                    panic!("overwrote existing player, this should never happen!");
                }
                true
            },
            PlayerLeave(player_leave) => {
                let player_id = player_leave.id;
                self.players.remove(&player_id).is_some()
            },
            PlayerRename(player_rename) => {
                let player_id = player_rename.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.name = player_rename.name.clone();
                    true
                } else {
                    false
                }
            },
            PlayerIncreaseScore(player_increase_score) => {
                let player_id = player_increase_score.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.score += player_increase_score.score;
                    true
                } else {
                    false
                }
            },
            PlayerConnect(player_connect) => {
                let player_id = player_connect.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    if !player.connected {
                        player.connected = true;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            PlayerDisconnect(player_disconnect) => {
                let player_id = player_disconnect.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    if player.connected {
                        player.connected = false;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            _ => {
                // if reached here means we can ignore the event
                // since it doesn't produce a state change
                false
            }
        }
    }
}

struct ServerRoomState {
    passes: HashMap<String, u32>,
}

impl ServerRoomState {
    fn new() -> Self {
        Self {
            passes: HashMap::new(),
        }
    }
}

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(query): Query<ConnectQuery>,
    State(rooms): State<Rooms>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("{user_agent} w/name {} w/pass {} w/room {} at {addr} connected", query.name, query.pass, query.room);

    // this code is written in such a clunky way to
    // work around limitations in borrow checker
    let room_tx = {
        let mut maybe_room_tx = None;
        if let Some(room) = rooms.0.get(&query.room) {
            println!("found room {}", query.room);
            maybe_room_tx = Some(room.room_tx.clone());
        }
        if maybe_room_tx.is_none() {
            println!("creating room {}", query.room);
            let (game_tx, _) = broadcast::channel::<SerializedMsg>(256);
            let (room_tx, room_rx) = mpsc::channel::<RoomEvent>(256);
            let room = Room::new(game_tx.clone(), room_tx.clone());
            rooms.0.insert(query.room.clone(), room);
            tokio::spawn(room_manager(rooms, query.room.clone(), game_tx, room_rx));
            maybe_room_tx = Some(room_tx);
        }
        maybe_room_tx.expect("found or created room")
    };

    // 1. player has to register themself with room
    let (register_tx, register_rx) = oneshot::channel();
    room_tx
        .send(RoomEvent::PlayerConnect {
            respond: register_tx,
            connect: query.clone()
        })
        .await
        .unwrap(); // TODO: this panics if players leave and rejoin room
    let recv_result = register_rx.await;
    if let Ok(register_result) = recv_result {
        match register_result {
            Ok((player_id, register_event, game_rx)) => {
                ws.on_upgrade(move |socket| player_manager(
                    socket, 
                    addr,
                    player_id,
                    register_event,
                    game_rx,
                    room_tx,
                ))
            },
            Err(error) => {
                error_into_response(error, query)
            }
        }
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "failed to register {} w/pass {} to room {}",
                query.name,
                query.pass,
                query.room,
            ),
        ).into_response()
    }
}

fn serialize_set_game_state(
    mut game_state: GameState,
    mut events_buf: Vec<ServerEvent>,
    player_id: u32,
) -> (GameState, Vec<ServerEvent>, UniqueSerializedMsg) {
    events_buf.push(ServerEvent {
        r#type: Some(server_event::Type::SetGameState(ServerEventSetGameState {
            player_id: player_id,
            game_state: Some(game_state),
        }))
    });
    let server_events = ServerEvents {
        events: events_buf,
    };
    let mut serialized = Vec::with_capacity(256);
    server_events
        .encode(&mut serialized)
        .expect("was able to encode set game state msg");
    // get buffer back out
    events_buf = server_events.events;
    // get game state back out
    game_state = {
        let server_event = events_buf
            .pop()
            .unwrap();
        if let Some(server_event::Type::SetGameState(server_game_state)) = server_event.r#type {
            server_game_state.game_state.unwrap()
        } else {
            panic!("this should be impossible")
        }
    };
    (game_state, events_buf, serialized)
}

impl ServerEvent {
    fn from_player(player_id: PlayerId, player_event: PlayerEvent) -> Self {
        use generated_rust::player_event;
        match player_event.r#type.unwrap() {
            player_event::Type::Rename(rename) => {
                ServerEvent {
                    r#type: Some(server_event::Type::PlayerRename(ServerEventPlayerRename {
                        id: player_id,
                        name: rename.name,
                    })),
                }
            },
            player_event::Type::IncreaseScore(increase_score) => {
                ServerEvent {
                    r#type: Some(server_event::Type::PlayerIncreaseScore(ServerEventPlayerIncreaseScore {
                        id: player_id,
                        score: increase_score.score,
                    })),
                }
            },
        }
    }
}

fn serialize_server_events(
    server_event: ServerEvent,
    mut events_buf: Vec<ServerEvent>
) -> (Vec<ServerEvent>, SerializedMsg) {
    events_buf.push(server_event);
    let server_events = ServerEvents {
        events: events_buf,
    };
    let mut serialized = Vec::<u8>::with_capacity(32);
    server_events
        .encode(&mut serialized)
        .expect("serialized server events");
    events_buf = server_events.events;
    events_buf.clear();
    (events_buf, Arc::new(serialized))
}

async fn room_manager(rooms: Rooms, room_name: String, game_tx: GameTx, mut room_rx: RoomRx) {
    let mut next_player_id = 0u32;
    let mut game_state = GameState::new();
    let mut room_state = ServerRoomState::new();
    let mut events_buf: Vec<ServerEvent> = Vec::with_capacity(4);
    loop {
        let event = room_rx.recv().await;
        if event.is_none() {
            println!("room {room_name} got none from room_rx, manager closing");
            break;
        }
        use generated_rust::server_event::Type;
        println!("room got event {event:?}");
        match event.unwrap() {
            RoomEvent::PlayerConnect { respond, connect } => {
                let ConnectQuery { name, pass, room } = connect;
    
                // check if this an existing player reconnecting
                if let Some(&player_id) = room_state.passes.get(&pass) {

                    // let other players know this player has reconnected
                    let player_connect = ServerEvent {
                        r#type: Some(Type::PlayerConnect(ServerEventPlayerConnect {
                            id: player_id,
                        })),
                    };
        
                    // not sure when this would ever be false, maybe if player
                    // disconnected due to stale connection earlier?
                    let player_connected = game_state.advance(&player_connect);
                    debug_assert!(player_connected, "player {player_id} connected to room but was already connected in game state");
                    if player_connected {
                        let (
                            returned_events_buf,
                            serialized_msg
                        ) = serialize_server_events(
                            player_connect,
                            events_buf,
                        );
                        events_buf = returned_events_buf;
                        game_tx
                            .send(serialized_msg)
                            .expect("sent player connect msg");
                    }

                    // send connected player current game state
                    let (
                        returned_game_state,
                        returned_events_buf,
                        serialized_msg
                    ) = serialize_set_game_state(
                        game_state,
                        events_buf,
                        player_id,
                    );
                    game_state = returned_game_state;
                    events_buf = returned_events_buf;
                    respond
                        .send(Ok((player_id, serialized_msg, game_tx.subscribe())))
                        .expect("sent init msg to reconnecting player");

                // otherwise this is a new player connecting
                } else {

                    // let other players know this player has joined
                    let new_player_id = next_player_id;
                    next_player_id += 1;
                    let player_join = ServerEvent {
                        r#type: Some(Type::PlayerJoin(ServerEventPlayerJoin {
                            id: new_player_id,
                            name: name,
                        })),
                    };
                    let is_very_first_player = new_player_id == 0;

                    // not sure when this would ever be false, something very wrong
                    // must have occurred for this to somehow be false
                    let player_joined = game_state.advance(&player_join);
                    debug_assert!(player_joined, "new player {new_player_id} connected but was already present in game state");
                    if player_joined {
                        room_state.passes.insert(pass, new_player_id);

                        // no need to send this event if there are no other
                        // players yet, which is why we check if this is the
                        // first player
                        if !is_very_first_player {
                            let (
                                returned_events_buf,
                                serialized_msg
                            ) = serialize_server_events(
                                player_join,
                                events_buf,
                            );
                            events_buf = returned_events_buf;
                            game_tx
                                .send(serialized_msg)
                                .expect("sent player join msg");
                        }
                    }

                    // sent joined player current game state
                    let (
                        returned_game_state,
                        returned_events_buf,
                        serialized_msg
                    ) = serialize_set_game_state(
                        game_state,
                        events_buf,
                        new_player_id,
                    );
                    game_state = returned_game_state;
                    events_buf = returned_events_buf;
                    respond
                        .send(Ok((new_player_id, serialized_msg, game_tx.subscribe())))
                        .expect("sent init msg to reconnecting player");
                }
            },
            RoomEvent::PlayerEvent {
                player_id,
                player_event 
            } => {
                let server_event = ServerEvent::from_player(player_id, player_event);
                let advanced = game_state.advance(&server_event);
                if advanced {
                    let (
                        returned_events_buf,
                        serialized_msg
                    ) = serialize_server_events(
                        server_event,
                        events_buf,
                    );
                    events_buf = returned_events_buf;
                    game_tx
                        .send(serialized_msg)
                        .expect("sent server event to all players");
                }
            },
            RoomEvent::PlayerDisconnect { player_id } => {
                let server_event = ServerEvent {
                    r#type: Some(server_event::Type::PlayerDisconnect(ServerEventPlayerDisconnect {
                        id: player_id,
                    })),
                };
                let advanced = game_state.advance(&server_event);
                if advanced {
                    println!("{room_name} game state {game_state:?}");
                    if game_state.empty() {
                        // destroy room
                        println!("destroying room {room_name}");
                        rooms.0.remove(&room_name);
                        break;
                    }
                    let (
                        returned_events_buf,
                        serialized_msg
                    ) = serialize_server_events(
                        server_event,
                        events_buf,
                    );
                    events_buf = returned_events_buf;
                    if let Err(err) = game_tx.send(serialized_msg) {
                        // if we're here it means all players have disconnected
                        // destroy room
                        println!("destroying room {room_name}");
                        rooms.0.remove(&room_name);
                        break;
                    }
                }
            },
        }
    }
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn player_manager(
    mut socket: WebSocket,
    who: SocketAddr,
    player_id: u32,
    set_game_state: UniqueSerializedMsg,
    game_rx: GameRx,
    room_tx: RoomTx,
) {
    let inactive_duration = Duration::from_millis(4000);
    let stale_duration = Duration::from_millis(9000);
    let alive_duration = Duration::from_millis(5000);
    let mut alive_interval = time::interval(alive_duration);
    let mut last_player_event = Instant::now();

    socket
        .send(Message::Binary(Bytes::from(set_game_state)))
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
                let diff = now.duration_since(last_player_event);
                // println!("alive check for {player_id}, now {now:?}, last_player_event {last_player_event:?}, diff {diff:?}");
                if diff > stale_duration {
                    println!("dropping stale player {player_id} @ {who}");
                    room_tx
                        .send(RoomEvent::PlayerDisconnect {
                            player_id,
                        })
                        .await
                        .expect("sent disconnect msg");
                    break;
                }
                if diff > inactive_duration {
                    let send_result = socket.send(Message::Ping(Bytes::from_static(b""))).await;
                    if let Err(err) = send_result {
                        println!("failed to send ping to player {player_id} @ {who}: {err}");
                        room_tx
                            .send(RoomEvent::PlayerDisconnect {
                                player_id,
                            })
                            .await
                            .expect("sent disconnect msg");
                        break;
                    }
                }
            },
            maybe_msg = socket.recv() => {
                last_player_event = Instant::now();
                if let Some(result_msg) = maybe_msg {
                    if let Ok(msg) = result_msg {
                        match msg {
                            Message::Binary(bytes) => {
                                match PlayerEvent::decode(bytes) {
                                    Ok(player_event) => {
                                        room_tx
                                            .send(RoomEvent::PlayerEvent {
                                                player_id,
                                                player_event,
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
                                    .send(RoomEvent::PlayerDisconnect {
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
                        println!("{player_id} @ {who} disconnected abruptly: {}", result_msg.unwrap_err());
                        room_tx
                            .send(RoomEvent::PlayerDisconnect {
                                player_id,
                            })
                            .await
                            .expect("sent disconnect msg");
                        break;
                    }
                } else {
                    println!("{player_id} @ {who} closed connection");
                    room_tx
                        .send(RoomEvent::PlayerDisconnect {
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

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket_old(mut socket: WebSocket, who: SocketAddr) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket
        .send(Message::Ping(Bytes::from_static(b"ping from server")))
        .await
        .is_ok()
    {
        println!("Pinged {who}...");
    } else {
        println!("Could not send ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            println!("client {who} abruptly disconnected");
            return;
        }
    }

    // Since each client gets individual statemachine, we can pause handling
    // when necessary to wait for some external event (in this case illustrated by sleeping).
    // Waiting for this client to finish getting its greetings does not prevent other clients from
    // connecting to server and receiving their greetings.
    for i in 1..5 {
        if socket
            .send(Message::Text(format!("Hi {i} times!").into()))
            .await
            .is_err()
        {
            println!("client {who} abruptly disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let n_msg = 10;
        for i in 0..n_msg {
            // In case of any websocket error, we exit.
            if sender
                .send(Message::Text(format!("Server message {i} ...").into()))
                .await
                .is_err()
            {
                return i;
            }

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        println!("Sending close to {who}...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Utf8Bytes::from_static("Goodbye"),
            })))
            .await
        {
            println!("Could not send Close due to {e}, probably it is ok?");
        }
        n_msg
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => println!("{a} messages sent to {who}"),
                Err(a) => println!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {b} messages"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    println!("Websocket context {who} destroyed");
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {who} sent str: {t:?}");
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
