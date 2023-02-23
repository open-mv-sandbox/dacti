use std::{any::Any, sync::mpsc::Sender as StdSender};

use stewart::Dispatcher;
use tracing::{event, Level};

use crate::runtime::AnyMessage;

pub struct NativeDispatcher {
    sender: StdSender<AnyMessage>,
}

impl NativeDispatcher {
    pub fn new(sender: StdSender<AnyMessage>) -> NativeDispatcher {
        Self { sender }
    }
}

impl Dispatcher for NativeDispatcher {
    fn send_any(&self, address: usize, message: Box<dyn Any>) {
        // TODO: Consider downcasting at this point to bin messages in contiguous queues,
        // maybe even avoiding the need for Box altogether by granting a memory slot in-line.

        let message = AnyMessage::new(address, message);
        let result = self.sender.send(message);

        // TODO: What to do with a send failure?
        if let Err(error) = result {
            event!(Level::ERROR, "failed to send message\n{:?}", error);
        }
    }
}
