use std::{
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

use futures::Stream;

use crate::{
    channels::{channel, Receiver},
    steps::Step,
};

pub struct ShellStream<T> {
    shared_state: Arc<Mutex<SharedState<T>>>,
}

struct SharedState<T> {
    receiver: Receiver<T>,
    waker: Option<Waker>,
    send_step: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl<T> Stream for ShellStream<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if let Some(send_step) = shared_state.send_step.take() {
            send_step();
        }

        match shared_state.receiver.try_receive() {
            Ok(Some(next)) => Poll::Ready(Some(next)),
            Ok(None) => {
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
            Err(_) => Poll::Ready(None),
        }
    }
}

impl<Op, Ev> crate::capability::CapabilityContext<Op, Ev>
where
    Op: crate::capability::Operation,
    Ev: 'static,
{
    /// Send an effect request to the shell, expecting a stream of responses
    pub fn stream_from_shell(&self, operation: Op) -> ShellStream<Op::Output> {
        let (sender, re