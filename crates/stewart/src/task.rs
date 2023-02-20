//! Handler bootstrappers and one-off operations.
//!
//! Handlers cannot initialize themselves, and some operations do not need to receive messages.
//! This is where tasks come in.

use anyhow::Result;
use tracing::{event, Level};

use crate::{Context, Handler};

/// Recipe for a task, which may optionally spawn a message handler.
pub struct Recipe {
    task: Box<dyn FnOnce(Context) -> Result<()>>,
}

impl Recipe {
    pub fn new<F>(task: F) -> Self
    where
        F: FnOnce(Context) -> Result<()> + 'static,
    {
        // TODO: Recipes taking a function limits them to only running locally. To properly
        // distribute the actor system, they should be able to be started remotely too.
        // Maybe that should be a different system entirely though?

        Self {
            task: Box::new(task),
        }
    }
}

pub struct TaskHandler;

impl Handler for TaskHandler {
    type Message = Recipe;

    fn handle(&mut self, context: &Context, message: Recipe) {
        let result = (message.task)(context.clone());

        // TODO: Report in some way to the caller where appropriate
        if let Err(error) = result {
            event!(Level::ERROR, "error while running task\n{:?}", error);
        }
    }
}
