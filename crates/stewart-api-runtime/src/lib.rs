//! Common stewart APIs for communicating with a runtime.
//!
//! These messages are not required to be used or supported, but for interoperability you should
//! prefer these over custom messages.

use anyhow::Error;
use stewart::ActorOps;

/// Start an actor on a runtime, using a factory function.
///
/// TODO: This only works on the same process, do we want to make this more generic, or have a
/// strict split between actors we can start locally and remotely? This is important for, for
/// example, communicating with a web-worker in-browser.
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
