use std::sync::{mpsc::Sender as StdSender, Arc};

use anyhow::Error;
use stewart::{Actor, Next};
use stewart_local::StartActor;

use crate::{actors::Actors, runtime::AnyMessage, sender::NativeSender};

pub struct StarterActor {
    sender: StdSender<AnyMessage>,
    actors: Arc<Actors>,
}

impl StarterActor {
    pub fn new(sender: StdSender<AnyMessage>, actors: Arc<Actors>) -> Result<Self, Error> {
        Ok(Self { sender, actors })
    }
}

impl Actor for StarterActor {
    type Message = StartActor;

    fn handle(&mut self, message: Self::Message) -> Result<Next, Error> {
        let factory = |id| {
            let sender = NativeSender::new(id, self.sender.clone());
            message.run_factory(sender)
        };
        self.actors.start(factory);

        Ok(Next::Continue)
    }
}
