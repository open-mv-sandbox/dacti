//! Backing types for implementing a runtime.

use std::any::Any;

use crate::handler::AnyHandler;

pub trait RuntimeContext: Send + Sync {
    fn send(&self, mailbox: usize, message: Box<dyn Any + Send>);

    fn add_handler(&self, handler: Box<dyn AnyHandler>) -> usize;
}
