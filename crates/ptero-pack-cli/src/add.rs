use std::fs::{File, OpenOptions};

use anyhow::{Context, Error};
use clap::Args;
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
    let mut package = OpenOptions::new()
        .read(true)
        .write(true)
        .open(command.package)
        .context("failed to open target package for writing")?;

    let mut input = File::open(&command.input)?;

    ptero_pack::add_file(&mut package, &mut input, command.uuid)?;

    Ok(())
}
