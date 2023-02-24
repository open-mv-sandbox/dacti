use std::any::Any;

use crate::Factory;

/// Shared dispatcher implementation for local runtime actions.
///
/// Dispatchers allow you to create one shared implementation for a set of functions, specific to
/// a particular backend. This avoids having to allocate and track a `Box` for every instance.
pub trait Dispatcher {
    /// Send a message on behalf of the actor to the target address.
    fn send(&self, actor_id: usize, address: usize, message: Box<dyn Any>);

    /// Start a new child actor on behalf of the actor.
    fn start(&self, actor_id: usize, factory: Box<dyn Factory>);
}
