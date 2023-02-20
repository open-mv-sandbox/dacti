//! Native runtime for stewart.

use std::{any::Any, sync::Arc};

use crossbeam::queue::SegQueue;
use sharded_slab::Slab;
use stewart::{
    runtime::{DowncastMailboxHandler, RuntimeContext},
    Context,
};
use tracing::{event, Level};

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
            self.handle_message(message);
        }
    }

    fn handle_message(&self, message: Message) {
        // TODO: Send addressing error back to handler
        let result = self.context_impl.mailboxes.get(message.address);
        let mailbox = match result {
            Some(handler) => handler,
            None => {
                event!(Level::ERROR, "failed to find mailbox at address");
                return;
            }
        };

        let result = self.context_impl.handlers.get(mailbox.handler);
        let handler = match result {
            Some(handler) => handler,
            None => {
                event!(Level::ERROR, "failed to find handler for mailbox");
                return;
            }
        };

        // Run the handler
        let result = handler.handle(&self.context, mailbox.state.as_ref(), message.message);

        // TODO: If a handler fails, maybe it should stop/restart the handler?
        if let Err(error) = result {
            event!(Level::ERROR, "error in handler\n{:?}", error);
        }
    }
}

#[derive(Default)]
struct RuntimeContextImpl {
    queue: SegQueue<Message>,
    handlers: Slab<Box<dyn DowncastMailboxHandler>>,
    mailboxes: Slab<Mailbox>,
}

impl RuntimeContext for RuntimeContextImpl {
    fn send(&self, address: usize, message: Box<dyn Any + Send>) {
        self.queue.push(Message { address, message });
    }

    fn add_handler(&self, handler: Box<dyn DowncastMailboxHandler>) -> usize {
        self.handlers
            .insert(handler)
            .expect("unable to insert new handler")
    }

    fn add_mailbox(&self, handler: usize, state: Box<dyn Any + Send + Sync>) -> usize {
        let mailbox = Mailbox { handler, state };
        self.mailboxes
            .insert(mailbox)
            .expect("unable to insert new mailbox")
    }
}

struct Message {
    address: usize,
    message: Box<dyn Any + Send>,
}

struct Mailbox {
    handler: usize,
    state: Box<dyn Any + Send + Sync>,
}
