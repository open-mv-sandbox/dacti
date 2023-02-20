//! One-off asynchronous operation executor.
//!
//! Not all asynchronous operations require an ongoing handler, for example:
//! - Handler initialization
//! - CPU intensive calculations
//!
//! This module adds an optional task handlers, which can execute these one-off asynchronous
//! operations within the actor runtime.

use anyhow::{Error, Result};
use tracing::{event, Level};

use crate::{Context, Handler};

/// Task handler that executes a task immediately on the same thread.
pub struct ImmediateTaskHandler;

impl Handler for ImmediateTaskHandler {
    type Message = Task;

    fn handle(&mut self, context: &Context, message: Task) -> Result<(), Error> {
        let result = (message.task)(context.clone());

        // Log task failure
        // TODO: Instead, we should probably notify the calling handler
        if let Err(error) = result {
            event!(Level::ERROR, "error while running task\n{:?}", error);
        }

        Ok(())
    }
}

/// Wrapping container for a task function.
pub struct Task {
    task: Box<dyn FnOnce(Context) -> Result<()> + Send>,
}

impl Task {
    pub fn new<F>(task: F) -> Self
    where
        F: FnOnce(Context) -> Result<()> + 'static + Send,
    {
        // TODO: Recipes taking a function limits them to only running locally. To properly
        // distribute the actor system, they should be able to be started remotely too.
        // Maybe that should be a different system entirely though?

        Self {
            task: Box::new(task),
        }
    }
}
