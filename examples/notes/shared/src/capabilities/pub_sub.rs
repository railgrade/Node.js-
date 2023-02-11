use crux_core::capability::{Capability, CapabilityContext, Operation};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

// TODO add topics

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PubSubOperation {
    Publish(Vec<u8>),
    Subscribe,
}

#[derive(Deserialize)]
pub struct Message(Vec<u8>);

impl Operation for PubSubOperation {
    type Output = Message;
}

pub struct PubSub<Event> {
    context: CapabilityContext<PubSubOperation, Event>,
}

impl<Ev> PubSub<Ev>
where
    Ev: 'static,
{
    pub fn new(context: CapabilityContext<PubSubOperation, Ev>) -> Self {
        Self { context }
    }

    pub fn subscribe<F>(&self, make_event: F)
    where
        F: Fn(Vec<u8>) -> Ev + Clone + Send + 'static,
    {
        self.context.spawn({
            let context = self.context.clone();

            async move {
                let mut stream = context.stream_from_shell(PubSubOperation::Subscribe);

                while let Some(message) = stream.next().await {
    