use std::sync::Arc;

use anyhow::Error;
use stewart::{local::StartActor, Actor, Next};

use crate::{actors::Actors, dispatcher::NativeDispatcher};

pub struct StarterActor {
    actors: Arc<Actors>,
    dispatcher: Arc<NativeDispatcher>,
}

impl StarterActor {
    pub fn new(actors: Arc<Actors>, dispatcher: Arc<NativeDispatcher>) -> Result<Self, Error> {
        Ok(Self { actors, dispatcher })
    }
}

impl Actor for StarterActor {
    type Message = StartActor;

    fn handle(&mut self, message: StartActor) -> Result<Next, Error> {
        let factory = |id| message.create(id, self.dispatcher.clone());
        self.actors.start(factory);

        Ok(Next::Continue)
    }
}
