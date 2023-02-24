//! "Common Use Case APIs" for starting and communicating with actors locally.
//!
//! ## Common Use Case APIs
//!
//! These APIs are made available in a common standard to cover the 'common use case' for APIs
//! that benefit from being interoperable. They are not required to be used, but are recommended
//! when they do cover your use case.
//!
//! Runtimes may expose these APIs to provide functionality libraries can make use of, without
//! depending on a specific runtime.

mod actor;
mod context;
mod dispatcher;

use std::{
    marker::PhantomData,
    sync::{atomic::AtomicPtr, Arc},
};

use anyhow::Error;

pub use self::{actor::AnyActor, context::Context, dispatcher::Dispatcher};
pub use stewart_local_derive::Factory;

/// Opaque target address of an actor.
pub struct Address<M> {
    address: usize,
    _m: PhantomData<AtomicPtr<M>>,
}

impl<M> Address<M> {
    pub fn from_raw(address: usize) -> Self {
        Self {
            address,
            _m: PhantomData,
        }
    }
}

impl<M> Clone for Address<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for Address<M> {}

/// Instructions for creating an actor on a runtime locally.
pub trait Factory {
    fn start(
        self: Box<Self>,
        actor_id: usize,
        dispatcher: Arc<dyn Dispatcher>,
        address: usize,
    ) -> Result<Box<dyn AnyActor>, Error>;
}
