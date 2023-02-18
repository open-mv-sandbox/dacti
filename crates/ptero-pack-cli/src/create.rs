use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};

use anyhow::{Context, Error};
use clap::Args;
use dacti_pack::IndexComponentHeader;
use daicon::{ComponentEntry, ComponentTableHeader};
use uuid::uuid;

/// Create a new dacti package.
#[derive(Args, Debug)]
pub struct CreateCommand {
    /// The path to create the package at.
    #[arg(short, long, value_name = "PATH")]
    package: String,
}

pub fn run(command: CreateCommand) -> Result<(), Error> {
    println!("creating package...");

    // Reserve 1kb for header and component table
    let indices_offset: u32 = 1024;

    // Open the target file, overwriting anything already there
    let mut package = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(command.package)
        .context("failed to open target package for writing")?;

    // Write the signature
    package.write_all(&daicon::SIGNATURE)?;

    // Write the component table
    let mut header = ComponentTableHeader::new();
    header.set_length(1);
    package.write_all(header.as_bytes())?;

    let mut entry = ComponentEntry::new();
    entry.set_type_uuid(uuid!("2c5e4717-b715-429b-85cd-d320d242547a"));

    let mut data = [0u8; 8];
    data[0..4].copy_from_slice(&indices_offset.to_le_bytes());
    data[4..8].copy_from_slice(&4u32.to_le_bytes());
    entry.set_data(data);

    package.write_all(entry.as_bytes())?;

    // Write an empty indices table
    package.seek(SeekFrom::Start(indices_offset as u64))?;
    let header = IndexComponentHeader::new();
    package.write_all(header.as_bytes())?;

    Ok(())
}
