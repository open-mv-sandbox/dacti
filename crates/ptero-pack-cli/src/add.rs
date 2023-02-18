use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};

use anyhow::{Context, Error};
use clap::Args;

/// Add files to a dacti package.
#[derive(Args, Debug)]
pub struct AddCommand {
    /// The path of the package to add files to.
    #[arg(short, long, value_name = "PATH")]
    package: String,

    /// The file to add.
    #[arg(short, long, value_name = "PATH")]
    file: String,
}

pub fn run(command: AddCommand) -> Result<(), Error> {
    println!("adding file to package...");

    // The first 32kb is reserved for components and indices
    let data_start = 1024 * 32;

    // Open the target package
    let mut package = OpenOptions::new()
        .write(true)
        .open(command.package)
        .context("failed to open target package for writing")?;

    // TODO: Find a free slot rather than just assuming there's no files yet
    // TODO: Update the index table

    // Write the file to the package
    package.seek(SeekFrom::Start(data_start))?;
    let data = std::fs::read(&command.file)?;
    package.write_all(&data)?;

    Ok(())
}
