//! Backing types for implementing a runtime.

use std::{any::Any, marker::PhantomData, sync::atomic::AtomicPtr};

use anyhow::Error;
use tracing::{event, Level};

use crate::{Context, Mailbox};

pub trait RuntimeContext {
    fn send(&self, address: usize, message: Box<dyn Any + Send>);

    fn register_handler(&self, handler: Box<dyn Any>) -> usize;

    fn register_mailbox(&self, handler: usize, executor: Box<dyn MailboxDowncastExecutor>)
        -> usize;
}

/// Downcasting mailbox executor.
///
/// Decodes messages and handlers back to concrete types, and then calls the handler's mailbox.
pub trait MailboxDowncastExecutor: Send + Sync {
    fn handle(
        &self,
        ctx: &Context,
        handler: &mut dyn Any,
        message: Box<dyn Any>,
    ) -> Result<(), Error>;
}

pub(crate) struct MailboxDowncastExecutorImpl<M: Any, H: Mailbox<M>> {
    _m: PhantomData<AtomicPtr<M>>,
    _h: PhantomData<AtomicPtr<H>>,
}

impl<M: Any, H: Mailbox<M>> Default for MailboxDowncastExecutorImpl<M, H> {
    fn default() -> Self {
        Self {
            _m: PhantomData,
            _h: PhantomData,
        }
    }
}

impl<M: Any, H: Mailbox<M>> MailboxDowncastExecutor for MailboxDowncastExecutorImpl<M, H> {
    fn handle(
        &self,
        ctx: &Context,
        handler: &mut dyn Any,
        message: Box<dyn Any>,
    ) -> Result<(), Error> {
        let handler_result = handler.downcast_mut::<H>();
        let message_result = message.downcast::<M>();

        match (handler_result, message_result) {
            (Some(handler), Ok(message)) => handler.handle(ctx, *message),
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
