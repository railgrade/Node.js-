use anyhow::{bail, Result};
use async_std::{
    fs::{File, OpenOptions},
    io::{ReadExt, WriteExt},
};
use bcs::{from_bytes, to_bytes};
use chrono::{DateTime, Utc};
use clap::Parser;
use shared::{
    http::protocol::{HttpRequest, HttpResponse},
    key_value::{KeyValueOperation, KeyValueOutput},
    platform::PlatformResponse,
    time::TimeResponse,
    Effect, Event, Request, ViewModel,
};
use std::{collections::VecDeque, time::SystemTime};

enum CoreMessage {
    Event(Event),
    Response(Vec<u8>, Outcome),
}

#[derive(Parser, Clone)]
enum Command {
    