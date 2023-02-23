use std::{
    any::Any,
    sync::{mpsc::Sender as StdSender, Arc},
};

use stewart::AnySender;
use tracing::{event, Level};

use crate::runtime::AnyMessage;

pub struct NativeSender {
    id: usize,
    sender: StdSender<AnyMessage>,
}

impl NativeSender {
    pub fn new(id: usize, sender: StdSender<AnyMessage>) -> Arc<NativeSender> {
        Arc::new(Self { id, sender })
    }
}

impl AnySender for NativeSender {
    fn send_any(&self, message: Box<dyn Any>) {
        // TODO: Now that we have backing types for senders, we could potentially have
        // type-specific 'bins' rather than one message channel, or even directly deliver to the
        // actor that's being sent to and just notify the executor.

        let message = AnyMessage::new(self.id, message);
        let result = self.sender.send(message);

        // TODO: What to do with a send failure?
        if let Err(error) = result {
            event!(Level::ERROR, "failed to send message\n{:?}", error);
        }
    }
}
