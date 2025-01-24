use axum::{extract::FromRef, routing::get, Router};
use drawduel_server::game::mini;
use std::{net::SocketAddr, process};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    // format!("{}=trace,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
                    format!("{}=trace", env!("CARGO_CRATE_NAME")).into()
                }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    println!("cwd {:?}", std::env::current_dir());

    // build our application with some routes
    let app =
        Router::new()
            .fallback_service(
                ServeDir::new("../../frontend/dist")
                    .append_index_html_on_directories(true),
            )
            .route("/mini-game-ws", get(mini::ws_handler))
            .with_state(SharedGlobalState::new())
            // logging so we can see whats going on
            .layer(TraceLayer::new_for_http().make_span_with(
                DefaultMakeSpan::default().include_headers(true),
            ));

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
