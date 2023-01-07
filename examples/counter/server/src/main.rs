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
use 