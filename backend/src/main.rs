#![allow(unused_imports, dead_code, unused_variables)]

use axum::{
    body::Bytes,
    extract::{ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade}, FromRef, Query, State},
    response::IntoResponse,
    routing::{any, get},
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

use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

use futures::{sink::SinkExt, stream::StreamExt};

use drawduel::game::mini;

#[derive(Clone)]
struct SharedGlobalState {
    mini_game: mini::SharedServiceState,
}

impl SharedGlobalState {
    fn new() -> Self {
        SharedGlobalState {
            mini_game: mini::SharedServiceState::new(),
        }
    }
}

impl FromRef<SharedGlobalState> for mini::SharedServiceState {
    fn from_ref(shared_global_state: &SharedGlobalState) -> Self {
        shared_global_state.mini_game.clone()
    }
}

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
        .route("/mini-game-ws", get(mini::ws_handler))
        .with_state(SharedGlobalState::new())
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:42069")
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
