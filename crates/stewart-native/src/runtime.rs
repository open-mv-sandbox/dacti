use std::any::Any;

use crossbeam::queue::SegQueue;
use sharded_slab::Slab;
use stewart::{handler::AnyHandler, ActorOps};
use tracing::{event, Level};

// TODO: Run threaded on a thread pool runtime like tokio.

/// Local blocking handler execution runtime.
pub struct Runtime {
    queue: SegQueue<Message>,
    handlers: Slab<Box<dyn AnyHandler>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            queue: Default::default(),
            handlers: Default::default(),
        }
    }

    /// Temporary helper while actors cannot be spawned yet.
    #[deprecated]
    pub fn run_with_ops(&self, callback: impl FnOnce(&dyn ActorOps)) {
        let ops = NativeActorOps { runtime: self };
        callback(&ops);
    }

    /// Execute handlers until no messages remain.
    pub fn block_execute(&self) {
        while let Some(message) = self.queue.pop() {
            self.handle_message(message);
        }
    }

    fn handle_message(&self, message: Message) {
        // TODO: Send addressing error back to handler
        let result = self.handlers.get(message.address);
        let actor = match result {
            Some(handler) => handler,
            None => {
                event!(Level::ERROR, "failed to find actor for address");
                return;
            }
        };

        // Run the handler
        let ops = NativeActorOps { runtime: self };
        let result = actor.handle(&ops, message.message);

        // TODO: If a handler fails, maybe it should stop/restart the handler?
        if let Err(error) = result {
            event!(Level::ERROR, "error in handler\n{:?}", error);
        }
    }
}

/// Actor operations wrapper.
struct NativeActorOps<'a> {
    runtime: &'a Runtime,
}

impl<'a> ActorOps for NativeActorOps<'a> {
    fn add_handler_any(&self, handler: Box<dyn AnyHandler>) -> usize {
        self.runtime
            .handlers
            .insert(handler)
            .expect("unable to insert handler")
    }

    fn send_any(&self, address: usize, message: Box<dyn Any + Send>) {
        self.runtime.queue.push(Message { address, message });
    }
}

struct Message {
    address: usize,
    message: Box<dyn Any + Send>,
}
