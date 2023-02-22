use anyhow::Error;
use stewart::{ActorOps, Handler, Next};
use stewart_api_runtime::StartActor;

pub struct StartActorHandler;

impl Handler for StartActorHandler {
    type Message = StartActor;

    fn handle(&self, ops: &dyn ActorOps, message: Self::Message) -> Result<Next, Error> {
        // TODO: Actually manage actors, this just runs the handlers in-line
        // TODO: Do something with errors
        message.run_factory(ops)?;

        Ok(Next::Continue)
    }
}
