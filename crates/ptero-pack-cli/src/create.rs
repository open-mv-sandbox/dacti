use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
    mem::size_of,
};

use anyhow::{Context, Error};
use clap::Args;
use dacti_pack::{IndexComponentHeader, INDEX_COMPONENT_UUID};
use daicon::{ComponentEntry, ComponentTableHeader, RegionData};
use tracing::{event, Level};

/// Create a new dacti package.
#[derive(Args, Debug)]
pub struct CreateCommand {
    /// The path to create the package at.
    #[arg(short, long, value_name = "PATH")]
    package: String,
}

pub fn run(command: CreateCommand) -> Result<(), Error> {
    event!(Level::INFO, "creating package...");

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
    let mut header = ComponentTableHeader::zeroed();
    header.set_length(1);
    package.write_all(header.as_bytes())?;

    let mut entry = ComponentEntry::zeroed();
    entry.set_type_uuid(INDEX_COMPONENT_UUID);

    let region = RegionData::from_bytes_mut(entry.data_mut());
    region.set_offset(indices_offset);
    region.set_size(size_of::<IndexComponentHeader>() as u32);

    package.write_all(entry.as_bytes())?;

    // Write an empty indices table
    package.seek(SeekFrom::Start(indices_offset as u64))?;
    let header = IndexComponentHeader::zeroed();
    package.write_all(header.as_bytes())?;

    Ok(())
}
