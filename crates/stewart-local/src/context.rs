use std::{any::Any, sync::Arc};

use crate::{dispatcher::Dispatcher, Address, Factory};

/// Context for a specific actor's execution.
///
/// This handle lets you perform some operations on a specific actor, like starting child actors,
/// or getting a message sender to that actor.
pub struct Context {
    actor_id: usize,
    dispatcher: Arc<dyn Dispatcher>,
}

impl Context {
    /// Create a typed context from raw actor ID and dispatcher.
    ///
    /// You generally do not have to do this yourself, instead prefer using the Factory derive
    /// macro.
    pub fn from_raw(actor_id: usize, dispatcher: Arc<dyn Dispatcher>) -> Self {
        Self {
            actor_id,
            dispatcher,
        }
    }

    /// Send a message from this actor to a target address.
    pub fn send<T>(&self, address: Address<T>, message: T)
    where
        T: Any,
    {
        let message = Box::new(message);
        self.dispatcher
            .send(self.actor_id, address.address, message);
    }

    /// Start a new child actor.
    pub fn start(&self, factory: impl Factory + 'static) {
        self.dispatcher.start(self.actor_id, Box::new(factory));
    }
}
