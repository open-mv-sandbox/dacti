//! A minimal modular actor interface.
//!
//! Stewart is built to allow for runtimes that distribute execution on both native and web
//! targets, and communicate with various async executors, even within the same process.
//!
//! This is a reference documentation for stewart, for more detailed explanation on stewart's
//! design philosophy, read the stewart book.

use std::{
    any::Any,
    marker::PhantomData,
    sync::{atomic::AtomicPtr, Arc},
};

use anyhow::Error;

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

/// Type-safe generic message sender, to send messages to any actor regardless of sending
/// implementation.
pub struct Sender<M> {
    // TODO: It would be nice if this could somehow be in-line and not an indirection.
    sender: Arc<dyn AnySender>,
    _m: PhantomData<AtomicPtr<M>>,
}

impl<M: Any> Sender<M> {
    pub fn from_any_sender(sender: Arc<dyn AnySender>) -> Self {
        Self {
            sender,
            _m: PhantomData,
        }
    }

    pub fn send(&self, message: M) {
        self.sender.send_any(Box::new(message));
    }
}

impl<M> Clone for Sender<M> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            _m: PhantomData,
        }
    }
}

pub trait AnySender {
    fn send_any(&self, message: Box<dyn Any>);
}
