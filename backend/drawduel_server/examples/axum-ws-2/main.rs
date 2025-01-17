use axum::{
    body::Bytes,
    extract::{ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade}, Query},
    response::IntoResponse,
    routing::any,
    Router,
};
use axum_extra::TypedHeader;
use serde::Deserialize;
use tokio::{sync::mpsc, time::{self, sleep}};

use std::{ops::ControlFlow, time::{Duration, Instant}};
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

#[derive(Deserialize)]
struct ConnectQuery {
    name: String,
    pass: String,
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
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("{user_agent} w/name {} & w/pass {} at {addr} connected", query.name, query.pass);
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    let recv_every = Duration::from_millis(500);
    let mut health_interval = time::interval(recv_every);
    // health_interval.tick().await; // first tick happens instantly
    let mut last_recv = Instant::now();
    let (tx, mut rx) = mpsc::channel::<Message>(32);
    let sender = tokio::spawn(async move {
        for i in 0..10 {
            let msg = Message::Text(format!("server msg {i}").into());
            let send_result = tx.send(msg).await;
            if let Err(err) = send_result {
                println!("err when sending msg: {err}");
                break;
            }
            sleep(Duration::from_millis(500)).await;
        }
    });
    let mut waiting_for_pong = false;
    loop {
        tokio::select! {
            // note to self: double pings seem to be bad,
            // if sent a ping always wait for a pong, do
            // not send a 2nd ping, and if a pong doesn't
            // return after some timeout consider the connection
            // stale and can drop it without a close message,
            // since the client didn't respond to our ping
            // it's not like it would see the close message anyway
            _ = health_interval.tick() => {
                let now = Instant::now();
                let since_last_recv = now.duration_since(last_recv);
                if since_last_recv > (recv_every * 3) {
                    println!("dropping stale connection {who}");
                    break;
                }
                if !waiting_for_pong && since_last_recv > recv_every {
                    let send_result = socket.send(Message::Ping(Bytes::from_static(b""))).await;
                    waiting_for_pong = true;
                    if let Err(err) = send_result {
                        println!("failed to send ping to {who}: {err}");
                        break;
                    }
                }
            },
            maybe_msg = socket.recv() => {
                last_recv = Instant::now();
                waiting_for_pong = false;
                if let Some(result_msg) = maybe_msg {
                    if let Ok(msg) = result_msg {
                        if process_message(msg, who).is_break() {
                            break;
                        }
                    } else {
                        println!("{who} disconnected abruptly: {}", result_msg.unwrap_err());
                        break;
                    }
                } else {
                    println!("{who} closed connection");
                    break;
                }
            },
            // this will return None if the channel has been
            // closed and won't send anymore messages, but
            // if we were to match on None this handler will
            // turn into a busy hot loop since None is always
            // return immediately for a closed channel, instead
            // we match on Some(msg) to tell tokio to ignore
            // the Nones when selecting an arm
            Some(msg_to_send) = rx.recv() => {
                // if let Some(msg_to_send) = maybe_msg_to_send {
                    let send_result = socket.send(msg_to_send).await;
                    if let Err(err) = send_result {
                        println!("failed to send msg to {who}: {err}");
                        break;
                    }
                // } else {
                //     println!("in none branch, hot loop?");
                // }
            },
        }
    }
    if let Err(err) = sender.await {
        println!("sender failed to join: {err}");
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
