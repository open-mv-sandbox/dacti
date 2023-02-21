//! A minimal modular actor interface.
//!
//! Stewart is built to allow for runtimes that distribute execution on both native and web
//! targets, and communicate with various async executors, even within the same process.
//!
//! ## Stewart Compared to Async-Await
//!
//! Stewart's "actors" can be seen as analogous to "futures", but more flexible and more explicit
//! in their runtime design. What you would usually do using async-await, in stewart you would use
//! chained "handler"s for. This makes stewart a bit heavier on boilerplate than async-await, but
//! also more direct and explicit about its functionality and overhead.
//!
//! Currently, stewart handler chains also have a performance penalty over async-await. This isn't
//! inherent in the pattern, and should be resolved in the future.

mod context;
pub mod handler;
pub mod runtime;

use std::{marker::PhantomData, sync::atomic::AtomicPtr};

pub use self::context::Context;

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
