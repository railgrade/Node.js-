use async_std::io::ReadExt;
use crossbeam_channel::Sender;
use futures::{stream, TryStreamExt};

use bcs::{from_bytes, to_bytes};
use clap::Parser;
use eyre::{bail, eyre, Result};
use shared::{
    http::protocol::{HttpHeader, HttpRequest, HttpResponse},
    sse::{SseRequest, SseResponse},
    Effect, Event, Request, ViewModel,
};
use std::{
    str::FromStr,
    sync::{Arc, Weak},
    time::Duration,
};
use surf::{http::Method, Client, Config, Url};

enum CoreMessage {
    Event(Event),
    Response(Vec<u8>, Outcome),
}

#[derive(Parser, Clone)]
enum Command {
    Get,
    Inc,
    Dec,
    Watch,
}

impl From<Command> for CoreMessage {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Get => CoreMessage::Event(Event::Get),
            Command::Inc => CoreMessage::Event(Event::Increment),
            Command::Dec => CoreMessage::Event(Event::Decrement),
            Command::Watch => CoreMessage::Event(Event::StartWatch),
        }
    }
}

pub enum Outcome {
    Http(HttpResponse),
    Sse(SseResponse),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

fn main() -> Result<()> {
    let (tx, rx) = crossbeam_c