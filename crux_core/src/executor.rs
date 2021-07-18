use std::{
    sync::{Arc, Mutex},
    task::Context,
};

use crossbeam_channel::{Receiver, Sender};
use futures::{
    future,
    task::{waker_ref, ArcWake},
    Future, FutureExt,
};

pub(crate) struct QueuingExecutor {
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
pub struct Spawner {
    task_sender: Sender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<future::BoxFuture<'static, ()>>>,

    task_sender: Sender<Arc<Task>>,
}

pub(crate) fn executor_and_spawner() -> (QueuingExecutor, Spawner) {
    let (task_sender, ready_queue) = crossbeam_channel::unbounded();

    (QueuingExecutor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });

        self.task_s