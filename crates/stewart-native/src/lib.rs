//! Native runtime for stewart.

use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crossbeam::queue::SegQueue;
use sharded_slab::Slab;
use stewart::{Address, Context, Handler, Mailbox};

// TODO: Run threaded on a thread pool runtime like tokio.

/// Local blocking handler execution runtime.
pub struct Runtime {
    mailbox: Arc<MailboxImpl>,
    context: Context,
    handlers: Slab<Mutex<Box<dyn DynHandler>>>,
}

impl Runtime {
    pub fn new() -> Self {
        let mailbox = Arc::new(MailboxImpl::default());
        let context = Context::new(mailbox.clone());

        Self {
            mailbox,
            context,
            handlers: Slab::new(),
        }
    }

    pub fn add_handler<H: Handler + 'static>(&self, handler: H) -> Address<H::Message> {
        let handler = wrap_handler(handler);
        let address = self.handlers.insert(handler).unwrap();
        Address::from_raw(address)
    }

    pub fn send<M: Any>(&self, address: Address<M>, message: M) {
        self.context.send(address, message);
    }

    /// Execute handlers until no messages remain.
    pub fn block_execute(&self) {
        while let Some(message) = self.mailbox.queue.pop() {
            // TODO: Handle failed addressing gracefully
            let handler = self.handlers.get(message.address).unwrap();
            let mut handler = handler.lock().unwrap();
            handler.handle(&self.context, message.message);
        }
    }
}

fn wrap_handler<H: Handler + 'static>(handler: H) -> Mutex<Box<dyn DynHandler>> {
    Mutex::new(Box::new(DynHandlerImpl { handler }))
}

trait DynHandler {
    fn handle(&mut self, context: &Context, message: Box<dyn Any>);
}

struct DynHandlerImpl<H> {
    handler: H,
}

impl<H: Handler> DynHandler for DynHandlerImpl<H> {
    fn handle(&mut self, context: &Context, message: Box<dyn Any>) {
        // TODO: Handle failed downcast gracefully
        let message = *message.downcast().expect("failed to downcast message");
        self.handler.handle(context, message);
    }
}

#[derive(Default)]
struct MailboxImpl {
    queue: SegQueue<Envelope>,
}

impl Mailbox for MailboxImpl {
    fn send(&self, address: usize, message: Box<dyn Any>) {
        let envelope = Envelope { address, message };
        self.queue.push(envelope);
    }
}

struct Envelope {
    address: usize,
    message: Box<dyn Any>,
}
