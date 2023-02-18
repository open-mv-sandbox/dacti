use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
    path::Path,
};

use anyhow::{Context, Error};
use clap::Args;
use daicon::{ComponentEntry, ComponentTableHeader};
use uuid::uuid;

#[derive(Args, Debug)]
pub struct CreateCommand {
    #[arg(short, long, value_name = "PATH")]
    package: String,
}

pub fn run(command: CreateCommand) -> Result<(), Error> {
    let package = Path::new(&command.package);

    // This tool will in the future contain command line options for altering packages, but for
    // now is just a hardcoded test tool.
    let test_data = include_bytes!("../../../data/shader.wgsl");

    // Reserve 1kb for header and component table
    let indices_offset: u32 = 1024;

    // Open the target file, overwriting anything already there
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(package)
        .context("failed to open target package for writing")?;

    // Write the signature
    file.write_all(&daicon::SIGNATURE)?;

    // Write the component table
    let mut header = ComponentTableHeader::new();
    header.set_count(1);
    file.write_all(header.as_bytes())?;

    let mut entry = ComponentEntry::new(uuid!("2c5e4717-b715-429b-85cd-d320d242547a"));

    let mut data = [0u8; 8];
    data[0..4].copy_from_slice(&indices_offset.to_le_bytes());
    data[4..8].copy_from_slice(&(test_data.len() as u32).to_le_bytes());
    entry.set_data(data);

    file.write_all(entry.as_bytes())?;

    // Write the indices
    file.seek(SeekFrom::Start(indices_offset as u64))?;
    // TODO: Actually write indices, for now we'rEe just writing a single block of data in-place
    file.write_all(test_data)?;

    Ok(())
}
