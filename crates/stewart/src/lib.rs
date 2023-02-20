//! A minimal modular actor interface.
//!
//! Allows for runtimes that distribute execution on both native and web targets, and various
//! executors.

mod context;
mod mailbox;
pub mod runtime;

use std::{marker::PhantomData, sync::atomic::AtomicPtr};

pub use self::{context::Context, mailbox::Actor};

pub struct Address<M> {
    address: usize,
    _p: PhantomData<AtomicPtr<M>>,
}

impl<M> Clone for Address<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for Address<M> {}
