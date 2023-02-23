use std::{
    any::Any,
    sync::{
        mpsc::{channel, Receiver, Sender as StdSender},
        Arc,
    },
};

use stewart::{Next, Sender};
use stewart_local::StartActor;
use tracing::{event, Level};

use crate::{actors::Actors, sender::NativeSender, starter::StarterActor};

/// Native stewart execution runtime.
pub struct NativeRuntime {
    sender: StdSender<AnyMessage>,
    receiver: Receiver<AnyMessage>,
    actors: Arc<Actors>,
}

impl NativeRuntime {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let actors = Arc::new(Actors::new());

        Self {
            sender,
            receiver,
            actors,
        }
    }

    /// Bootstrap the starter actor.
    pub fn start_starter(&self) -> Sender<StartActor> {
        let id = self
            .actors
            .start(|_| {
                let actor = StarterActor::new(self.sender.clone(), self.actors.clone())?;
                Ok(Box::new(actor))
            })
            .expect("failed to start StarterActor");

        let sender = NativeSender::new(id, self.sender.clone());
        Sender::from_any_sender(sender)
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
