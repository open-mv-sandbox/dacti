use std::{any::Any, sync::mpsc::Sender};

use stewart_local::Dispatcher;
use tracing::{event, Level};

use crate::runtime::AnyMessage;

pub struct NativeDispatcher {
    factory_runner_id: usize,
    sender: Sender<AnyMessage>,
}

impl NativeDispatcher {
    pub fn new(factory_runner_id: usize, sender: Sender<AnyMessage>) -> NativeDispatcher {
        Self {
            factory_runner_id,
            sender,
        }
    }
}

impl Dispatcher for NativeDispatcher {
    fn send(&self, _actor_id: usize, address: usize, message: Box<dyn Any>) {
        // TODO: Consider downcasting at this point to bin messages in contiguous queues,
        // maybe even avoiding the need for Box altogether by granting a memory slot in-line.

        let message = AnyMessage::new(address, message);
        let result = self.sender.send(message);

        // TODO: What to do with a send failure?
        if let Err(error) = result {
            event!(Level::ERROR, "failed to send message\n{:?}", error);
        }
    }

    fn start(&self, actor_id: usize, factory: Box<dyn stewart_local::Factory>) {
        self.send(actor_id, self.factory_runner_id, Box::new(factory));
    }
}
