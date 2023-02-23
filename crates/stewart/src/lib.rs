//! A minimal modular actor interface.
//!
//! Stewart is built to allow for runtimes that distribute execution on both native and web
//! targets, and communicate with various async executors, even within the same process.
//!
//! This is a reference documentation for stewart, for more detailed explanation on stewart's
//! design philosophy, read the stewart book.

pub mod local;
mod sender;

use anyhow::Error;

pub use self::sender::{Dispatcher, Sender};

/// Actor message handling trait.
pub trait Actor {
    type Message;

    fn handle(&mut self, message: Self::Message) -> Result<Next, Error>;
}

/// What should be done with the actor after returning from the message handler.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Next {
    Continue,
    Stop,
}
