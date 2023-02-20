//! Native runtime for stewart.

use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crossbeam::queue::SegQueue;
use sharded_slab::Slab;
use stewart::{
    runtime::{RuntimeContext, RuntimeHandler},
    Context,
};

// TODO: Run threaded on a thread pool runtime like tokio.

/// Local blocking handler execution runtime.
pub struct Runtime {
    context_impl: Arc<RuntimeContextImpl>,
    context: Context,
}

impl Runtime {
    pub fn new() -> Self {
        let context_impl = Arc::new(RuntimeContextImpl::default());
        let context = Context::from_runtime(context_impl.clone());

        Self {
            context_impl,
            context,
        }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Execute handlers until no messages remain.
    pub fn block_execute(&self) {
        while let Some(message) = self.context_impl.queue.pop() {
            // TODO: Handle failed addressing gracefully
            let handler = self.context_impl.handlers.get(message.address).unwrap();
            let mut handler = handler.lock().unwrap();
            handler.handle(&self.context, message.message);
        }
    }
}

#[derive(Default)]
struct RuntimeContextImpl {
    queue: SegQueue<RuntimeMessage>,
    handlers: Slab<Mutex<Box<dyn RuntimeHandler>>>,
}

impl RuntimeContext for RuntimeContextImpl {
    fn send(&self, address: usize, message: Box<dyn Any + Send>) {
        self.queue.push(RuntimeMessage { address, message });
    }

    fn add_handler(&self, handler: Box<dyn stewart::runtime::RuntimeHandler>) -> usize {
        // TODO: Graceful error handling
        let handler = Mutex::new(handler);
        self.handlers
            .insert(handler)
            .expect("unable to insert new handler")
    }
}

struct RuntimeMessage {
    address: usize,
    message: Box<dyn Any + Send>,
}
