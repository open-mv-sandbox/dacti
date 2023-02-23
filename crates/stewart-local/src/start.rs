use std::{
    any::{type_name, Any},
    marker::PhantomData,
    sync::{atomic::AtomicPtr, Arc},
};

use anyhow::Error;
use stewart::{Actor, AnySender, Next, Sender};
use tracing::{event, Level};

/// Start an actor on a runtime, using a factory function.
///
/// TODO: This only works on the same process, do we want to make this more generic, or have a
/// strict split between actors we can start locally and remotely? This is important for, for
/// example, communicating with a web-worker in-browser.
#[must_use = "actor will not be started until start message is sent"]
pub struct StartActor {
    factory: Box<dyn AnyActorFactory>,
}

impl StartActor {
    pub fn new<A, F>(factory: F) -> Self
    where
        A: Actor + 'static,
        F: FnOnce(Sender<A::Message>) -> Result<A, Error> + 'static,
    {
        let factory = ActorFactory {
            factory,
            _a: PhantomData,
        };

        Self {
            factory: Box::new(factory),
        }
    }

    pub fn run_factory(self, sender: Arc<dyn AnySender>) -> Result<Box<dyn AnyActor>, Error> {
        self.factory.create(sender)
    }
}

trait AnyActorFactory {
    fn create(self: Box<Self>, sender: Arc<dyn AnySender>) -> Result<Box<dyn AnyActor>, Error>;
}

struct ActorFactory<A, F> {
    factory: F,
    _a: PhantomData<AtomicPtr<A>>,
}

impl<A, F> AnyActorFactory for ActorFactory<A, F>
where
    A: Actor + 'static,
    F: FnOnce(Sender<A::Message>) -> Result<A, Error>,
{
    fn create(self: Box<Self>, sender: Arc<dyn AnySender>) -> Result<Box<dyn AnyActor>, Error> {
        let sender = Sender::from_any_sender(sender);
        let actor = (self.factory)(sender)?;
        Ok(Box::new(actor))
    }
}

/// Downcasting interface for sending dynamic messages to actors.
pub trait AnyActor {
    fn handle_any(&mut self, message: Box<dyn Any>) -> Result<Next, Error>;
}

impl<H> AnyActor for H
where
    H: Actor,
    H::Message: Any,
{
    fn handle_any(&mut self, message: Box<dyn Any>) -> Result<Next, Error> {
        // TODO: Can we bypass AnyHandler's dynamic casting by redesigning the runtime to have type
        // specific channels? This might also eliminate the need for boxes.
        let result = message.downcast::<H::Message>();

        match result {
            Ok(message) => self.handle(*message),
            _ => {
                // This is an error with the caller, not the handler.
                // TODO: Report error to caller

                let handler_name = type_name::<H>();
                event!(
                    Level::ERROR,
                    handler = handler_name,
                    "failed to downcast message"
                );

                Ok(Next::Continue)
            }
        }
    }
}
