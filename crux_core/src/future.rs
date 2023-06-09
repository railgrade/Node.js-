//! Async support for implementing capabilities
//!
use std::{
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

use futures::Future;

use crate::Step;

pub struct ShellRequest<T> {
    shared_state: Arc<Mutex<SharedState<T>>>,
}

struct SharedState<T> {
    result: Option<T>,
    waker: Option<Waker>,
    send_step: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl<T> Future for ShellRequest<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if let Some(send_step) = shared_state.send_step.take() {
            send_step();
        }

        match shared_state.result.take() {
            Some(result) => Poll::Ready(result),
            None => {
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

impl<Op, Ev> crate::capability::CapabilityContext<Op, Ev>
where
    Op: crate::capability::Operation,
    Ev: 'static,
{
    /// Send an effect request to the shell, expecting an output. The
    /// provided `operation` describes the effect input in a serialisable fashion,
    /// and must implement the [`Operation`](crate::capability::Operation) trait to declare the expected
    /// output type.
    ///
    /// `request_from_shell` is returns a future of the output, which can be
    /// `await`ed. You should only call this method inside an async task
    /// created with [`CapabilityContext::spawn`](crate::capability::CapabilityContext::spawn).
    pub fn request_from_shell(&self, operation: Op) -> ShellRequest<Op::Output> {
        let shared_state = Arc::new(Mutex::new(SharedState {
            result: None,
            waker: None,
            send_step: None,
        }));

        // Our callback holds a weak pointer to avoid circular references
        // from shared_state -> send_step -> step -> shared_state
        let callback_shared_state = Arc::downgrade(&shared_state);

        let step = Step::resolves_once(operation, move |bytes| {
            let Some(shared_state) = callback_shared_state.upgrade() else {
                // The ShellRequest was dropped before we were called, so just
                // do nothing.
                return;
            };

            let mut shared_state = shared_state.lock().unwrap();
            shared_state.result = Some(bcs::from_bytes(bytes).unwrap());
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        let send_step_context = self.clone();
        let send_step = move || send_step_context.send_step(step);

        shared_state.lock().unwrap().send_step = Some(Box::new(send_step));

        ShellRequest { shared_state }
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

    impl Operation for TestOperation {
        type Output = ();
    }

    #[test]
    fn test_effect_future() {
        let (step_sender, steps) = channel();
        let (event_sender, events) = channel::<()>();
        let (executor, spawner) = executor_and_spawner();
        let capability_context =
            CapabilityContext::new(step_sender, event_sender.clone(), spawner.clone());

        let future = capability_context.request_from_shell(TestOperation);

        // The future hasn't been awaited so we shouldn't have any steps.
        assert_matches!(steps.receive(), None);
        assert_matches!(events.receive(), None);

        // It also shouldn't have spawned anything so check that
        executor.run_all();
        assert_matches!(steps.receive(), None);
        assert_matches!(events.receive(), None);

        spawner.spawn(async move {
            future.await;
            event_sender.send(());
        });

        // We still shouldn't have any steps
        assert_matches!(steps.receive(), None);
        assert_matches!(events.receive(), None);

        executor.run_all();
        let step = steps.receive().expect("we should have a step here");
        assert_matches!(steps.receive(), None);
        assert_matches!(events.receive(), None);

        let Some(Resolve::Once(resolve)) = step.resolve else {
            panic!("Expected a resolve once");
        };
        resolve(&[]);
        assert_matches!(steps.receive(), None);
        assert_matches!(events.receive(), None);

        executor.run_all();
        assert_matches!(steps.receive(), None);
        assert_matches!(events.receive(), Some(()));
        assert_matches!(events.receive(), None);
    }
}
