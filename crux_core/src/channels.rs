// Wrappers around crossbeam_channel that only expose the functionality we need (and is safe on wasm)

use std::sync::Arc;

pub(crate) fn channel<T>() -> (Sender<T>, Receiver<T>)
where
    T: Send + 'static,
{
    let (sender, receiver) = crossbeam_channel::unbounded();
    let sender = Sender {
        inner: Arc::new(sender),
    };
    let receiver = Receiver { inner: receiver };

    (sender, receiver)
}

pub struct Receiver<T> {
    inner: crossbeam_channel::Receiver<T>,
}

impl<T> Receiver<T> {
    /// Receives a message if any are waiting.
    ///
    /// Panics if the receiver has disconnected, so shouldn't be used if
    /// that's possible.
    pub fn receive(&self) -> Option<T> {
        match self.inner.try_recv() {
            Ok(inner) => Some(inner),
            Err(crossbeam_channel::TryRecvError::Empty) => None,
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                // Users _generally_ shouldn't be messing with channels themselves, so
                // this probably shouldn't happen.  Might happen in tests, but lets
                // fix that if we get complaints
                panic!("Receiver was disconnected.")
            }
        }
    }

    /// Receives a message if any are waiting.
    /// Returns the error branch if the sender has disconnected.
    ///
    /// This API isn't that nice, but isn't intended for public consumption
    /// so whatevs.
    pub fn try_receive(&self) -> Result<Option<T>, ()> {
        match self.inner.try_recv() {
            Ok(inner) => Ok(Some(inner)),
            Err(crossbeam_channel::TryRecvError::Empty) => Ok(None),
            Err(c