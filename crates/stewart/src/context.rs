use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    runtime::{DowncastActorHandlerImpl, RuntimeContext},
    Actor, Address,
};

/// Runtime context handle.
#[derive(Clone)]
pub struct Context {
    context: Arc<dyn RuntimeContext>,
}

impl Context {
    pub fn from_runtime(context: Arc<dyn RuntimeContext>) -> Self {
        Self { context }
    }

    pub fn add_actor<A: Actor>(&self, actor: A) -> Address<A::Message> {
        let actor = DowncastActorHandlerImpl::new(actor);
        let address = self.context.add_actor(actor);
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
