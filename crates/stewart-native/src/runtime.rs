use std::any::Any;

use crossbeam::queue::SegQueue;
use sharded_slab::Slab;
use stewart::{handler::AnyHandler, ActorOps, Address};
use stewart_runtime::StartActor;
use tracing::{event, Level};

use crate::manager::StartActorHandler;

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

    pub fn send<M: Any + Send>(&self, address: Address<M>, message: M) {
        let message = Message {
            address: address.to_raw(),
            message: Box::new(message),
        };
        self.queue.push(message);
    }

    pub fn start_actor_manager(&self) -> Address<StartActor> {
        let ops = NativeActorOps { runtime: self };
        let ops: &dyn ActorOps = &ops;
        let address = ops.add_handler(StartActorHandler);

        address
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
