//! Stewart runtime APIs.
//!
//! These messages are not required to be used or supported, but for interoperability you should
//! prefer these over custom messages.

use anyhow::Error;
use stewart::ActorOps;

/// Start an actor on a runtime, using a factory function.
#[must_use = "actor will not be started until message is sent"]
pub struct StartActor {
    factory: Box<dyn FnOnce(&dyn ActorOps) -> Result<(), Error> + Send>,
}

impl StartActor {
    pub fn new<F: FnOnce(&dyn ActorOps) -> Result<(), Error> + Send + 'static>(factory: F) -> Self {
        Self {
            factory: Box::new(factory),
        }
    }

    pub fn run_factory(self, ops: &dyn ActorOps) -> Result<(), Error> {
        (self.factory)(ops)
    }
}
