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
        let (sender, receiver) = channel();
        let shared_state = Arc::new(Mutex::new(SharedState {
            receiver,
            waker: None,
            send_step: None,
        }));

        // Our callback holds a weak pointer so the channel can be freed
        // whenever the associated task ends.
        let callback_shared_state = Arc::downgrade(&shared_state);

        let step = Step::resolves_many_times(operation, move |bytes| {
            let Some(shared_state) = callback_shared_state.upgrade() else {
                // Let the StepRegistry know that the associated task has finished.
                return Err(());
            };

            let mut shared_state = shared_state.lock().unwrap();

            sender.send(bcs::from_bytes(bytes).unwrap());
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }

            Ok(())
        });

        // Put a callback into our shared_state so that we only send
        // our request to the shell when the stream is first polled.
        let send_step_context = self.clone();
        let send_step = move || send_step_context.send_step(step);
        shared_state.lock().unwrap().send_step = Some(Box::new(send_step));

        ShellStream { shared_state }
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::{
        capability::{CapabilityContext, Operation},
        channels::channel,
        executor::executor_and_spawner,
        steps::Resolve,
    };

    #[derive(serde::Serialize, PartialEq, Eq, Debug)]
    struct TestOperation;

    impl