//! Backing types for implementing a runtime.

use std::any::Any;

use anyhow::Error;
use tracing::{event, Level};

use crate::{Actor, Context};

pub trait RuntimeContext: Send + Sync {
    fn send(&self, mailbox: usize, message: Box<dyn Any + Send>);

    fn add_actor(&self, actor: Box<dyn DowncastActorHandler>) -> usize;
}

/// Downcasting mailbox executor.
///
/// Decodes messages and handlers back to concrete types, and then calls the handler's mailbox.
pub trait DowncastActorHandler: Send + Sync {
    fn handle(&self, ctx: &Context, message: Box<dyn Any>) -> Result<(), Error>;
}

pub(crate) struct DowncastActorHandlerImpl<H> {
    handler: H,
}

impl<H: Actor> DowncastActorHandlerImpl<H> {
    pub fn new(handler: H) -> Box<dyn DowncastActorHandler> {
        Box::new(Self { handler })
    }
}

impl<H: Actor> DowncastActorHandler for DowncastActorHandlerImpl<H> {
    fn handle(&self, ctx: &Context, message: Box<dyn Any>) -> Result<(), Error> {
        let result = message.downcast::<H::Message>();

        match result {
            Ok(message) => self.handler.handle(ctx, *message),
            _ => {
                // This is an error with the caller, not the handler.
                // In fact, this should be prevented by address type guard.
                // If this does happen, figure out why and fix or document it.

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
