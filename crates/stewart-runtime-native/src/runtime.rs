use std::{
    any::Any,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
};

use stewart::Next;
use stewart_local::{Address, Dispatcher, StartActor};
use tracing::{event, Level};

use crate::{actors::Actors, manager::StartActorActor};

/// Native stewart execution runtime.
pub struct NativeRuntime {
    dispatcher: Arc<NativeDispatcher>,
    receiver: Receiver<AnyMessage>,
    actors: Arc<Actors>,
}

impl NativeRuntime {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let dispatcher = Arc::new(NativeDispatcher(sender));
        let actors = Arc::new(Actors::new());

        Self {
            dispatcher,
            receiver,
            actors,
        }
    }

    pub fn dispatcher(&self) -> &Arc<NativeDispatcher> {
        &self.dispatcher
    }

    pub fn start_actor_manager(&self) -> Address<StartActor> {
        let address = self
            .actors
            .start(|_| {
                let actor = StartActorActor::new(self.actors.clone())?;
                Ok(Box::new(actor))
            })
            .expect("failed to start start actor");

        Address::from_raw(address)
    }

    /// Execute handlers until no messages remain.
    pub fn block_execute(&self) {
        // TODO: Execution should happen on a thread pool.
        // This has some implications for handler locking that should be checked at that point.
        // For example, task scheduling should be done in a way that avoids mutex lock contention.
        // Maybe execution workers should just be given handlers to run from the scheduler, rather
        // than messages? Then there's no need for mutexes at all.

        // TODO: Message executor as actor?
        // Per-message-type actors won't work, as we very frequently want to distribute the same
        // message across multiple threads.
        while let Ok(message) = self.receiver.try_recv() {
            self.handle_message(message);
        }
    }

    fn handle_message(&self, message: AnyMessage) {
        // Run the actor's handler
        let result = self
            .actors
            .run(message.address, |actor| actor.handle_any(message.message));

        // TODO: What should we do with the error?
        let next = match result {
            Ok(Ok(next)) => next,
            Err(error) => {
                event!(Level::ERROR, "error while finding actor\n{:?}", error);
                return;
            }
            Ok(Err(error)) => {
                event!(
                    Level::ERROR,
                    "error while running actor.handle\n{:?}",
                    error
                );
                return;
            }
        };

        // If the actor wants to remove itself, remove it
        if next == Next::Stop {
            self.actors.stop(message.address);
        }
    }
}

pub struct AnyMessage {
    address: usize,
    message: Box<dyn Any>,
}

pub struct NativeDispatcher(Sender<AnyMessage>);

impl Dispatcher for NativeDispatcher {
    fn send_any(&self, address: usize, message: Box<dyn Any>) {
        let message = AnyMessage { address, message };
        let result = self.0.send(message);

        // TODO: What to do with a send failure?
        if let Err(error) = result {
            event!(Level::ERROR, "failed to send message\n{:?}", error);
        }
    }
}
