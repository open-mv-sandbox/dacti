//! Backing types for implementing a runtime.

use std::any::Any;

use anyhow::Error;
use tracing::{event, Level};

use crate::{Context, MailboxHandler};

pub trait RuntimeContext {
    fn send(&self, mailbox: usize, message: Box<dyn Any + Send>);

    fn add_handler(&self, handler: Box<dyn DowncastMailboxHandler>) -> usize;

    fn add_mailbox(&self, handler: usize, state: Box<dyn Any + Send + Sync>) -> usize;
}

/// Downcasting mailbox executor.
///
/// Decodes messages and handlers back to concrete types, and then calls the handler's mailbox.
pub trait DowncastMailboxHandler: Send + Sync {
    fn handle(&self, ctx: &Context, state: &dyn Any, message: Box<dyn Any>) -> Result<(), Error>;
}

pub(crate) struct DowncastMailboxHandlerImpl<H> {
    handler: H,
}

impl<H: MailboxHandler> DowncastMailboxHandlerImpl<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
}

impl<H: MailboxHandler> DowncastMailboxHandler for DowncastMailboxHandlerImpl<H> {
    fn handle(&self, ctx: &Context, state: &dyn Any, message: Box<dyn Any>) -> Result<(), Error> {
        let state_result = state.downcast_ref::<H::State>();
        let message_result = message.downcast::<H::Message>();

        match (state_result, message_result) {
            (Some(state), Ok(message)) => self.handler.handle(ctx, state, *message),
            _ => {
                // This is an error with the caller, not the handler.
                // In fact, this should be prevented by address type guard.
                // If this does happen, figure out why and fix or document it.

                let handler_name = std::any::type_name::<H>();
                event!(
                    Level::ERROR,
                    handler = handler_name,
                    "failed to downcast message or handler"
                );

                Ok(())
            }
        }
    }
}
