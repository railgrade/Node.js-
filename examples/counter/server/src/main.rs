use axum::{
    extract::State,
    http::Method,
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    routing::{get, post},
    Json, Router, Server,
};
use futures::Stream;
use futures_signals::signal::{Mutable, SignalExt};
use serde::Serialize;
use std::{
    convert::Infallible,
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct CounterState {
    value: Mutable<Counter>,
}

#[derive(Serialize, Copy, Clone)]
struct Counter {
    value: isize,
    updated_at: u128,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = CounterState {
        value: Mutable::new(Counter {
            value: 0,
            updated_at: now(),
        }),
    };

    let app = Router::new()
        .route("/", get(get_counter))
        .route("/sse", get(sse_handler))
        .route("/inc", post(inc))
        .route("/dec", post(dec))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_counter(State(counter): State<CounterState>) -> impl IntoResponse {
    let value = counter.value.lock_mut();

    Json(*value)
}

async fn inc(State(counter): State<CounterState>) -> impl IntoResponse {
    let mut value = counter.value.lock_mut();
    let new = value.value.saturating_add(1);
   