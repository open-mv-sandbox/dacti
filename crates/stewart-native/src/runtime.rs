use std::{
    any::Any,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
};

use stewart::Next;
use stewart_local::Factory;
use tracing::{event, Level};

use crate::{actors::Actors, factory_runner::FactoryRunnerActor};

/// Native stewart execution runtime.
pub struct NativeRuntime {
    sender: Sender<AnyMessage>,
    receiver: Receiver<AnyMessage>,
    actors: Arc<Actors>,
    factory_runner_id: usize,
}

impl NativeRuntime {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let actors = Arc::new(Actors::new());

        let factory_runner_id = actors
            .start(|id| {
                let actor = FactoryRunnerActor::new(id, actors.clone(), sender.clone())?;
                Ok(Box::new(actor))
            })
            .expect("failed to start StarterActor");

        Self {
            sender,
            receiver,
            actors,
            factory_runner_id,
        }
    }

    pub fn start(&self, factory: Box<dyn Factory>) {
        let message = AnyMessage::new(self.factory_runner_id, Box::new(factory));
        self.sender.send(message).unwrap();
    }

    /// Execute handlers until no messages remain.
    pub fn block_execute(&self) {
        // TODO: This is a very naive way of running an async executor, and needs a lot of
        // improvement to work with external systems, because it can't wait on external signals.

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
            .run(message.id, |actor| actor.handle_any(message.message));

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
            self.actors.stop(message.id);
        }
    }
}

pub struct AnyMessage {
    id: usize,
    message: Box<dyn Any>,
}

impl AnyMessage {
    pub fn new(id: usize, message: Box<dyn Any>) -> Self {
        Self { id, message }
    }
}
