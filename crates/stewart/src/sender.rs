use std::{
    any::Any,
    marker::PhantomData,
    sync::{atomic::AtomicPtr, Arc},
};

/// Type-safe generic message sender, to send messages to any actor regardless of sending
/// implementation.
pub struct Sender<M> {
    address: usize,
    dispatcher: Arc<dyn Dispatcher>,
    _m: PhantomData<AtomicPtr<M>>,
}

impl<M: Any> Sender<M> {
    /// Create a sender from an opaque address value and dispatcher.
    ///
    /// The intent here is that a runtime implements one dispatcher for a type of sender, and uses
    /// the address value to resolve to a specific destination. This avoids needing to frequently
    /// allocate senders, and keeps the data more cache-local.
    ///
    /// We could potentially improve this further using 'sized dyn'/'dyn*' when/if that becomes
    /// possible, and leave the specific method entirely up to the implementor.
    pub fn from_raw(address: usize, dispatcher: Arc<dyn Dispatcher>) -> Self {
        Self {
            address,
            dispatcher,
            _m: PhantomData,
        }
    }

    pub fn send(&self, message: M) {
        self.dispatcher.send_any(self.address, Box::new(message));
    }
}

impl<M> Clone for Sender<M> {
    fn clone(&self) -> Self {
        Self {
            address: self.address,
            dispatcher: self.dispatcher.clone(),
            _m: PhantomData,
        }
    }
}

/// Dispatcher implementation that can send a message using a given opaque address.
pub trait Dispatcher {
    fn send_any(&self, address: usize, message: Box<dyn Any>);
}
