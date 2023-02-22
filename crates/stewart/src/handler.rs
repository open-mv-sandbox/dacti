use std::any::{type_name, Any};

use anyhow::Error;
use tracing::{event, Level};

use crate::ActorOps;

/// Addressable handler for a specific message receiving implementation.
pub trait Handler: Send + Sync + 'static {
    type Message: Any;

    fn handle(&self, ops: &dyn ActorOps, message: Self::Message) -> Result<Next, Error>;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Next {
    Continue,
    Stop,
}

/// Downcasting interface for sending dynamic messages to handlers.
pub trait AnyHandler: Send + Sync {
    fn handle(&self, ops: &dyn ActorOps, message: Box<dyn Any>) -> Result<Next, Error>;
}

impl<H: Handler> AnyHandler for H {
    fn handle(&self, ops: &dyn ActorOps, message: Box<dyn Any>) -> Result<Next, Error> {
        // TODO: Can we bypass AnyHandler's dynamic casting by redesigning the runtime to have type
        // specific channels? This might also eliminate the need for boxes.
        let result = message.downcast::<H::Message>();

        match result {
            Ok(message) => self.handle(ops, *message),
            _ => {
                // This is an error with the caller, not the handler.
                // TODO: Report error to caller

                let handler_name = type_name::<H>();
                event!(
                    Level::ERROR,
                    handler = handler_name,
                    "failed to downcast message"
                );

                Ok(Next::Continue)
            }
        }
    }
}
