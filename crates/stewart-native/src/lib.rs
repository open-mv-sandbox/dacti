//! Native runtime for stewart.

use std::{any::Any, sync::Arc};

use crossbeam::queue::SegQueue;
use sharded_slab::Slab;
use stewart::{
    handler::AnyHandler,
    runtime::{RuntimeHandle, RuntimeHandleInner},
};
use tracing::{event, Level};

// TODO: Run threaded on a thread pool runtime like tokio.

/// Local blocking handler execution runtime.
pub struct Runtime {
    handle_inner: Arc<NativeRuntimeHandleInner>,
    handle: RuntimeHandle,
}

impl Runtime {
    pub fn new() -> Self {
        let handle_inner = Arc::new(NativeRuntimeHandleInner::default());
        let handle = RuntimeHandle::from_inner(handle_inner.clone());

        Self {
            handle_inner,
            handle,
        }
    }

    pub fn handle(&self) -> &RuntimeHandle {
        &self.handle
    }

    /// Execute handlers until no messages remain.
    pub fn block_execute(&self) {
        while let Some(message) = self.handle_inner.queue.pop() {
            self.handle_message(message);
        }
    }

    fn handle_message(&self, message: Message) {
        // TODO: Send addressing error back to handler
        let result = self.handle_inner.handlers.get(message.address);
        let actor = match result {
            Some(handler) => handler,
            None => {
                event!(Level::ERROR, "failed to find actor for address");
                return;
            }
        };

        // Run the handler
        let result = actor.handle(message.message);

        // TODO: If a handler fails, maybe it should stop/restart the handler?
        if let Err(error) = result {
            event!(Level::ERROR, "error in handler\n{:?}", error);
        }
    }
}

#[derive(Default)]
struct NativeRuntimeHandleInner {
    queue: SegQueue<Message>,
    handlers: Slab<Box<dyn AnyHandler>>,
}

impl RuntimeHandleInner for NativeRuntimeHandleInner {
    fn add_handler(&self, handler: Box<dyn AnyHandler>) -> usize {
        self.handlers
            .insert(handler)
            .expect("unable to insert handler")
    }

    fn send(&self, address: usize, message: Box<dyn Any + Send>) {
        self.queue.push(Message { address, message });
    }
}

struct Message {
    address: usize,
    message: Box<dyn Any + Send>,
}
