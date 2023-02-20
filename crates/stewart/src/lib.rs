//! A minimal modular actor interface.
//!
//! Allows for runtimes that distribute execution on both native and web targets, and various
//! executors.

mod context;
pub mod runtime;
pub mod task;

use std::{any::Any, marker::PhantomData, sync::atomic::AtomicPtr};

use anyhow::Error;

pub use self::context::Context;

/// Mailbox message handler implementation.
///
/// TODO: State should maybe be synchronized inside the runtime, resolving locks by handling
/// other messages first. This would change state here to a mutable reference.
pub trait MailboxHandler: Send + Sync + 'static {
    type State: Any + Send + Sync;
    type Message: Any;

    fn handle(
        &self,
        ctx: &Context,
        state: &Self::State,
        message: Self::Message,
    ) -> Result<(), Error>;
}

pub struct HandlerAddress<M> {
    handler: usize,
    _p: PhantomData<AtomicPtr<M>>,
}

impl<M> Clone for HandlerAddress<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for HandlerAddress<M> {}

pub struct Address<M> {
    mailbox: usize,
    _p: PhantomData<AtomicPtr<M>>,
}

impl<M> Clone for Address<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for Address<M> {}
