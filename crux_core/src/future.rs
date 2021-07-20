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

        if let Some(send_step) = shar