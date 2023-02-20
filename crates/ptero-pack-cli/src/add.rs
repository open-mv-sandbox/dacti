use std::fs::OpenOptions;

use anyhow::{Context, Error};
use clap::Args;
use ptero_pack::create_add_data_recipe;
use stewart::task::ImmediateTaskHandler;
use stewart_native::Runtime;
use tracing::{event, Level};
use uuid::Uuid;

/// Add files to a dacti package.
#[derive(Args, Debug)]
pub struct AddCommand {
    /// The path of the package to add files to.
    #[arg(short, long, value_name = "PATH")]
    package: String,

    /// The input location of the file to add.
    #[arg(short, long, value_name = "PATH")]
    input: String,

    /// The UUID to assign the input file.
    #[arg(short, long, value_name = "UUID")]
    uuid: Uuid,
}

pub fn run(command: AddCommand) -> Result<(), Error> {
    event!(Level::INFO, "adding file to package...");

    // Open the target package
    let package = OpenOptions::new()
        .read(true)
        .write(true)
        .open(command.package)
        .context("failed to open target package for writing")?;

    let input = std::fs::read(&command.input)?;

    // Set up the runtime
    let runtime = Runtime::new();
    let task_handler = runtime.context().register(ImmediateTaskHandler);

    // TODO: Error not correctly bubbling up
    let recipe = create_add_data_recipe(package, input, command.uuid);
    runtime.context().send(task_handler, recipe);
    runtime.block_execute();

    Ok(())
}
