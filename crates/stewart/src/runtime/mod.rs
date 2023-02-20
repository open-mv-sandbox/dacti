//! Backing types for implementing a runtime.

use std::any::Any;

use crate::{Context, Handler};

pub trait RuntimeContext {
    fn send(&self, address: usize, message: Box<dyn Any + Send>);

    fn add_handler(&self, handler: Box<dyn RuntimeHandler>) -> usize;
}

/// Dynamic downcasting handler.
pub trait RuntimeHandler: Any + Send + Sync {
    fn handle(&mut self, context: &Context, message: Box<dyn Any>);
}

pub(crate) struct RuntimeHandlerImpl<H> {
    pub handler: H,
}

impl<H: Handler> RuntimeHandler for RuntimeHandlerImpl<H> {
    fn handle(&mut self, context: &Context, message: Box<dyn Any>) {
        // TODO: Graceful error handling
        let message = message.downcast().unwrap();
        self.handler.handle(context, *message);
    }
}
