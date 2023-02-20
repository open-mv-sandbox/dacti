//! A minimal modular actor interface.
//!
//! Allows for runtimes that distribute execution on both native and web targets, and various
//! executors.

pub mod task;

use std::{any::Any, marker::PhantomData, sync::Arc};

#[derive(Clone)]
pub struct Context {
    mailbox: Arc<dyn Mailbox>,
}

impl Context {
    pub fn new(mailbox: Arc<dyn Mailbox>) -> Self {
        Self { mailbox }
    }

    pub fn send<M: Any>(&self, address: Address<M>, message: M) {
        self.mailbox.send(address.address, Box::new(message));
    }
}

pub struct Address<M> {
    address: usize,
    _p: PhantomData<M>,
}

impl<M> Address<M> {
    pub fn from_raw(address: usize) -> Self {
        Self {
            address,
            _p: PhantomData,
        }
    }
}

pub trait Handler {
    type Message: Any;

    fn handle(&mut self, context: &Context, message: Self::Message);
}

pub trait Mailbox {
    fn send(&self, address: usize, message: Box<dyn Any>);
}
