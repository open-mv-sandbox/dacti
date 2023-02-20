//! Backing types for implementing a runtime.

use std::any::Any;

use anyhow::Error;
use tracing::{event, Level};

use crate::{Context, Handler};

pub trait RuntimeContext {
    fn send(&self, address: usize, message: Box<dyn Any + Send>);

    fn register(&self, handler: Box<dyn DowncastHandler>) -> usize;
}

/// Dynamic downcasting handler, to allow handlers to be sent across dynamic boundaries.
pub trait DowncastHandler: Send + Sync {
    fn handle(&mut self, ctx: &Context, message: Box<dyn Any>) -> Result<(), Error>;
}

pub(crate) struct RuntimeHandlerImpl<H> {
    pub handler: H,
}

impl<H: Handler> DowncastHandler for RuntimeHandlerImpl<H> {
    fn handle(&mut self, ctx: &Context, message: Box<dyn Any>) -> Result<(), Error> {
        let result = message.downcast::<H::Message>();
        match result {
            Ok(message) => self.handler.handle(ctx, *message),
            Err(_) => {
                // This is an error with the caller, not the handler.
                // In fact, this should be prevented by address type guard.
                // If this does happen, figure out why and fix or document it.

                let handler_name = std::any::type_name::<H>();
                event!(
                    Level::ERROR,
                    handler = handler_name,
                    "failed to downcast message for handler"
                );

                Ok(())
            }
        }
    }
}
