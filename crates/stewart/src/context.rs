use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    runtime::{RuntimeContext, RuntimeHandlerImpl},
    Address, Handler,
};

#[derive(Clone)]
pub struct Context {
    context: Arc<dyn RuntimeContext>,
}

impl Context {
    pub fn from_runtime(context: Arc<dyn RuntimeContext>) -> Self {
        Self { context }
    }

    /// Send a message to the handler at the address.
    pub fn send<M: Any + Send>(&self, address: Address<M>, message: M) {
        self.context.send(address.address, Box::new(message));
    }

    /// Register a handler, and return the address to send messages to it.
    pub fn register<H: Handler + 'static>(&self, handler: H) -> Address<H::Message> {
        let handler = Box::new(RuntimeHandlerImpl { handler });
        let address = self.context.register(handler);
        Address {
            address,
            _p: PhantomData,
        }
    }
}
