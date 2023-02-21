//! A minimal modular actor interface.
//!
//! Stewart is built to allow for runtimes that distribute execution on both native and web
//! targets, and communicate with various async executors, even within the same process.
//!
//! ## Actors
//!
//! In stewart, "actors" provide a hierarchical grouping of ongoing processes. They can be seen as
//! very similar to an operating system "process" in fact, in that the actor runtime will clean
//! up associated resources when the actor stops or fails.
//!
//! Actors also logically separate operations, communicating across message channels. This allows
//! for actors to be easily distributed between threads, cores, runtimes, and even separate
//! machines.
//!
//! Actors are also light-weight, comparable to a "future" in overhead, while providing more
//! explicit control over performance costs than a future does.
//!
//! TODO features for actors:
//! - grouping handlers under an actor
//! - hierarchical stop, if you stop one actor its children stop too
//!
//! ### When to use Actors
//!
//! Actors can be both short- or long-lived. The API provides no limitations or assumptions on
//! these use cases, these are just some examples.
//!
//! Short lived actors could be:
//! - Reading a file and returning it in-memory to the caller.
//! - Managing concurrent sub-steps of a CPU-intensive calculation.
//!
//! Long lived actors could be:
//! - Managing an ongoing connection to a replication server.
//! - Writing logging messages to a file asynchronously, using buffering as an optimization.
//!
//! ## Handlers
//!
//! Actors start and 'own' handlers, which can receive events on a specific address. Unlike some
//! other actor frameworks, stewart provides addresses per-handler rather than per-actor. This
//! lets actors start up handlers for specific callbacks, with bundled handler-specific data.
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

pub mod handler;
pub mod runtime;

use std::{marker::PhantomData, sync::atomic::AtomicPtr};

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
