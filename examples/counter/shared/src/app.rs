use crate::capabilities::sse::ServerSentEvents;
use chrono::{DateTime, NaiveDateTime, Utc};
use crux_core::render::Render;
use crux_http::Http;
use crux_macros::Effect;
use serde::{Deserialize, Serialize};
use url::Url;

const API_URL: &str = "https://crux-counter.fly.dev";

#[derive(Default)]
pub struct Model {
    count: Counter,
    confirmed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewModel {
    pub text: String,
    pub confirmed: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Event {
    // events from the shell
    Get,
    Increment,
    Decrement,
    StartWatch,

    // events local to the core
    #[serde(skip)]
    Set(crux_http::Result<crux_http::Response<Counter>>),
    WatchUpdate(Counter),
}

#[de