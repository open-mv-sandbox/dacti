use std::sync::Arc;

use anyhow::Error;
use stewart::{local::Factory, Actor, Next};

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
    type Message = Box<dyn Factory>;

    fn handle(&mut self, message: Box<dyn Factory>) -> Result<Next, Error> {
        // TODO: Track hierarchy
        let factory = |id| message.start(id, self.dispatcher.clone());
        self.actors.start(factory);

        Ok(Next::Continue)
    }
}
