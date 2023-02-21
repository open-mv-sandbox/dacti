use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{handler::Handler, runtime::RuntimeContext, Address};

/// Runtime context handle.
#[derive(Clone)]
pub struct Context {
    context: Arc<dyn RuntimeContext>,
}

impl Context {
    pub fn from_runtime(context: Arc<dyn RuntimeContext>) -> Self {
        Self { context }
    }

    // TODO: Associate handlers with actors, for tracking and cleanup
    pub fn add_handler<H: Handler>(&self, handler: H) -> Address<H::Message> {
        let handler = Box::new(handler);
        let address = self.context.add_handler(handler);
        Address {
            address,
            _p: PhantomData,
        }
    }

    /// Send a message to the handler at the address.
    pub fn send<M: Any + Send>(&self, address: Address<M>, message: M) {
        self.context.send(address.address, Box::new(message));
    }
}
