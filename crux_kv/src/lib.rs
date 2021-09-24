//! A basic Key-Value store for use with Crux
//!
//! `crux_kv` allows Crux apps to store and retrieve arbitrary data by asking the Shell to
//! persist the data using platform native capabilities (e.g. disk or web localStorage)
//!
//! This is still work in progress and extremely basic.
use crux_core::{
    capability::{CapabilityContext, Operation},
    Capability,
};
use serde::{Deserialize, Serialize};

/// Supported operations
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum KeyValueOperation {
    /// Read bytes stored under a key
    Read(String),
    /// Write bytes under a key
    Write(String, Vec<u8>),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum KeyValueOutput {
    // TODO: Add support for errors
    Read(Option<Vec<u8>>),
    // TODO: Add support for errors
    Write(bool),
}

impl Operation for KeyValueOperation {
    type Output = KeyValueOutput;
}

pub struct KeyValue<Ev> {
    context: CapabilityContext<KeyValueOperation, Ev>,
}

impl<Ev> KeyValue<Ev>
where
    Ev: 'static,
{
    pub fn new(context: CapabilityContext<KeyValueOperation, Ev>) -> Self {
        Self { context }
    }

    /// Read a value under `key`, will dispatch the event with a
    /// `KeyValueOutput::Read(Option<Vec<u8>>)` as payload
    pub fn read<F>(&self, key: &str, make_event: F)
    where
        F: Fn(KeyValueOutput) -> Ev + Send + Sync + 'static,
    {
        let ctx = self.context.clone();
        let key = key.to_string();
        self.context.spawn(async move {
            let output = ctx.request_from_shell(KeyValueOperation::Read(key)).await;

            ctx.update_app(make_event(output))
    