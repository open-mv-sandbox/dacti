use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    runtime::{DowncastMailboxHandlerImpl, RuntimeContext},
    Address, HandlerAddress, MailboxHandler,
};

#[derive(Clone)]
pub struct Context {
    context: Arc<dyn RuntimeContext>,
}

impl Context {
    pub fn from_runtime(context: Arc<dyn RuntimeContext>) -> Self {
        Self { context }
    }

    /// Add a mailbox handler implementation.
    pub fn add_handler<H: MailboxHandler>(&self, handler: H) -> HandlerAddress<H::Message> {
        let handler = DowncastMailboxHandlerImpl::new(handler);
        let handler = self.context.add_handler(Box::new(handler));

        HandlerAddress {
            handler,
            _p: PhantomData,
        }
    }

    /// Add a mailbox, linking handler to state.
    pub fn add_mailbox<M: Any, S: Any + Send + Sync>(
        &self,
        handler: HandlerAddress<M>,
        state: S,
    ) -> Address<M> {
        let mailbox = self.context.add_mailbox(handler.handler, Box::new(state));
        Address {
            mailbox,
            _p: PhantomData,
        }
    }

    /// Add a handler and immediately get a singular mailbox for it.
    pub fn add_one<H: MailboxHandler>(&self, handler: H, state: H::State) -> Address<H::Message> {
        let handler = self.add_handler(handler);
        self.add_mailbox(handler, state)
    }

    /// Send a message to the handler at the address.
    pub fn send<M: Any + Send>(&self, address: Address<M>, message: M) {
        self.context.send(address.mailbox, Box::new(message));
    }
}
