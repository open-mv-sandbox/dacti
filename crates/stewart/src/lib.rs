//! A minimal modular actor interface.
//!
//! Allows for runtimes that distribute execution on both native and web targets, and various
//! executors.

mod context;
mod handler;
pub mod runtime;
pub mod task;

use std::marker::PhantomData;

pub use self::{context::Context, handler::Handler};

pub struct Address<M> {
    address: usize,
    _p: PhantomData<M>,
}
