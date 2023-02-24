use std::any::{type_name, Any};

use anyhow::Error;
use stewart::{Actor, Next};
use tracing::{event, Level};

/// Downcasting interface for sending dynamic messages to actors.
pub trait AnyActor {
    fn handle_any(&mut self, message: Box<dyn Any>) -> Result<Next, Error>;
}

impl<H> AnyActor for H
where
    H: Actor,
    H::Message: Any,
{
    fn handle_any(&mut self, message: Box<dyn Any>) -> Result<Next, Error> {
        // TODO: Can we bypass AnyHandler's dynamic casting by redesigning the runtime to have type
        // specific channels? This might also eliminate the need for boxes.
        let result = message.downcast::<H::Message>();

        match result {
            Ok(message) => self.handle(*message),
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
