use std::sync::{mpsc::Sender, Arc};

use anyhow::Error;
use stewart::{Actor, Next};
use stewart_local::Factory;

use crate::{actors::Actors, dispatcher::NativeDispatcher, runtime::AnyMessage};

pub struct FactoryRunnerActor {
    actors: Arc<Actors>,
    dispatcher: Arc<NativeDispatcher>,
}

impl FactoryRunnerActor {
    pub fn new(id: usize, actors: Arc<Actors>, sender: Sender<AnyMessage>) -> Result<Self, Error> {
        let dispatcher = Arc::new(NativeDispatcher::new(id, sender));
        Ok(Self { actors, dispatcher })
    }
}

impl Actor for FactoryRunnerActor {
    type Message = Box<dyn Factory>;

    fn handle(&mut self, message: Box<dyn Factory>) -> Result<Next, Error> {
        // TODO: Track hierarchy
        let factory = |id| message.start(id, self.dispatcher.clone(), id);
        self.actors.start(factory);

        Ok(Next::Continue)
    }
}
