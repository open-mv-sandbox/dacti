use std::any::Any;

use crate::{
    handler::{AnyHandler, Handler},
    Address,
};

/// Trait for performing in-line operations on an actor.
pub trait ActorOps {
    fn add_handler_any(&self, handler: Box<dyn AnyHandler>) -> usize;

    fn send_any(&self, address: usize, message: Box<dyn Any + Send>);
}

impl dyn '_ + ActorOps {
    /// Add a new handler to an actor.
    /// TODO: Associate handlers with actors, for tracking and cleanup
    pub fn add_handler<H: Handler>(&self, handler: H) -> Address<H::Message> {
        let handler = Box::new(handler);
        let address = self.add_handler_any(handler);
        Address::from_usize(address)
    }

    /// Send a message to the handler at the address.
    /// TODO: Sender as its own type maybe?
    pub fn send<M: Any + Send>(&self, address: Address<M>, message: M) {
        self.send_any(address.to_usize(), Box::new(message));
    }
}
