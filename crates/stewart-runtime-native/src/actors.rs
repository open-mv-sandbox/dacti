use std::sync::Mutex;

use anyhow::{anyhow, Context, Error};
use sharded_slab::Slab;
use stewart_local::AnyActor;
use tracing::{event, Level};

/// Actors collection helper.
pub struct Actors {
    slab: Slab<Mutex<Box<dyn AnyActor>>>,
}

impl Actors {
    pub fn new() -> Self {
        Self {
            slab: Default::default(),
        }
    }

    /// Start an actor and return its address.
    pub fn start<F>(&self, factory: F) -> Option<usize>
    where
        F: FnOnce(usize) -> Result<Box<dyn AnyActor>, Error>,
    {
        event!(Level::TRACE, "starting actor");

        // Allocate an address
        let entry = self
            .slab
            .vacant_entry()
            .expect("unable to allocate actor address");
        let address = entry.key();

        // Attempt to create the actor
        let result = factory(address);
        let actor = match result {
            Ok(actor) => actor,
            Err(error) => {
                event!(Level::ERROR, "actor factory failed\n{:?}", error);
                return None;
            }
        };

        // Finalize the actor storage, and return its address
        entry.insert(Mutex::new(actor));
        Some(address)
    }

    pub fn stop(&self, address: usize) {
        event!(Level::TRACE, "stopping actor");
        self.slab.remove(address);
    }

    /// Run an operation on an actor by address.
    pub fn run<F, O>(&self, address: usize, action: F) -> Result<O, Error>
    where
        F: FnOnce(&mut dyn AnyActor) -> O,
    {
        // TODO: Send addressing error back to handler
        let actor = self
            .slab
            .get(address)
            .context("failed to find actor for address")?;
        let mut actor = actor.lock().map_err(|_| anyhow!("actor lock poisoned"))?;

        // Perform the action
        let result = action(actor.as_mut());

        Ok(result)
    }
}
