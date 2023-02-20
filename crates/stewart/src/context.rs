use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    runtime::{MailboxDowncastExecutorImpl, RuntimeContext},
    Address, Mailbox,
};

#[derive(Clone)]
pub struct Context {
    context: Arc<dyn RuntimeContext>,
}

impl Context {
    pub fn from_runtime(context: Arc<dyn RuntimeContext>) -> Self {
        Self { context }
    }

    /// Add a handler.
    ///
    /// Returns a type-safe builder for adding mailboxes to the handler.
    pub fn add<H: Any + Send + Sync>(&self, handler: H) -> HandlerBuilder<H> {
        let id = self.context.register_handler(Box::new(handler));
        HandlerBuilder {
            context: self,
            id,
            _h: PhantomData,
        }
    }

    /// Convenience helper for registering a handler with only one mailbox.
    ///
    /// Equivalent to `register(handler).add()`
    pub fn add_one<M: Any + Send, H: Mailbox<M>>(&self, handler: H) -> Address<M> {
        self.add(handler).add()
    }

    /// Send a message to the handler at the address.
    pub fn send<M: Any + Send>(&self, address: Address<M>, message: M) {
        self.context.send(address.address, Box::new(message));
    }
}

pub struct HandlerBuilder<'a, H: 'static> {
    context: &'a Context,
    id: usize,
    _h: PhantomData<H>,
}

impl<'a, H> HandlerBuilder<'a, H> {
    /// Register a dynamic mailbox, which relays messages to a handler.
    pub fn add<M: Any + Send>(&self) -> Address<M>
    where
        H: Mailbox<M>,
    {
        // TODO: Can we re-use executors, and will it help performance?
        // Maybe 'handlers' are an abstract runtime system, and users just specify
        // state + mailboxes?

        let executor = MailboxDowncastExecutorImpl::<M, H>::default();
        let address = self
            .context
            .context
            .register_mailbox(self.id, Box::new(executor));
        Address {
            address,
            _p: PhantomData,
        }
    }
}
