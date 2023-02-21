use std::any::Any;

use anyhow::Error;
use tracing::{event, Level};

use crate::Context;

/// Addressable handler for a specific message receiving implementation.
pub trait Handler: Send + Sync + 'static {
    type Message: Any;

    fn handle(&self, ctx: &Context, message: Self::Message) -> Result<(), Error>;
}

/// Downcasting interface for sending dynamic messages to handlers.
pub trait AnyHandler: Send + Sync {
    fn handle(&self, ctx: &Context, message: Box<dyn Any>) -> Result<(), Error>;
}

impl<H: Handler> AnyHandler for H {
    fn handle(&self, ctx: &Context, message: Box<dyn Any>) -> Result<(), Error> {
        let result = message.downcast::<H::Message>();

        match result {
            Ok(message) => self.handle(ctx, *message),
            _ => {
                // This is an error with the caller, not the handler.
                // TODO: Bubble up error

                let handler_name = std::any::type_name::<H>();
                event!(
                    Level::ERROR,
                    handler = handler_name,
                    "failed to downcast message"
                );

                Ok(())
            }
        }
    }
}
