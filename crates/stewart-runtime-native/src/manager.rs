use std::sync::Arc;

use anyhow::Error;
use stewart::{Actor, Next};
use stewart_local::StartActor;

use crate::actors::Actors;

pub struct StartActorActor {
    actors: Arc<Actors>,
}

impl StartActorActor {
    pub fn new(actors: Arc<Actors>) -> Result<Self, Error> {
        Ok(Self { actors })
    }
}

impl Actor for StartActorActor {
    type Message = StartActor;

    fn handle(&mut self, message: Self::Message) -> Result<Next, Error> {
        let factory = |address| message.run_factory(address);
        self.actors.start(factory);

        Ok(Next::Continue)
    }
}
