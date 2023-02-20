//! A minimal modular actor interface.
//!
//! Allows for runtimes that distribute execution on both native and web targets, and various
//! executors.

mod context;
pub mod runtime;
pub mod task;

use std::{any::Any, marker::PhantomData};

use anyhow::Error;

pub use self::context::Context;

pub trait Mailbox<M: Any>: Send + Sync + 'static {
    fn handle(&mut self, ctx: &Context, message: M) -> Result<(), Error>;
}

pub struct Address<M> {
    address: usize,
    _p: PhantomData<M>,
}

impl<M> Clone for Address<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for Address<M> {}
