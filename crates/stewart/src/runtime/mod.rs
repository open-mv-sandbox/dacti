//! Types for implementing and talking to a generic runtime.

use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    handler::{AnyHandler, Handler},
    Address,
};

/// Handle to spawn new actors and handler.
///
/// TODO: Maybe spawning actors and handlers should be separate?
/// It could be useful to spawn actors only through messages, and give actors a trait context to
/// spawn new handlers through they don't have to keep.
#[derive(Clone)]
pub struct RuntimeHandle {
    inner: Arc<dyn RuntimeHandleInner>,
}

impl RuntimeHandle {
    pub fn from_inner(inner: Arc<dyn RuntimeHandleInner>) -> Self {
        Self { inner }
    }

    // TODO: Associate handlers with actors, for tracking and cleanup
    pub fn add_handler<H: Handler>(&self, handler: H) -> Address<H::Message> {
        let handler = Box::new(handler);
        let address = self.inner.add_handler(handler);
        Address {
            address,
            _p: PhantomData,
        }
    }

    /// Send a message to the handler at the address.
    pub fn send<M: Any + Send>(&self, address: Address<M>, message: M) {
        self.inner.send(address.address, Box::new(message));
    }
}

pub trait RuntimeHandleInner: Send + Sync {
    fn add_handler(&self, handler: Box<dyn AnyHandler>) -> usize;

    fn send(&self, mailbox: usize, message: Box<dyn Any + Send>);
}
