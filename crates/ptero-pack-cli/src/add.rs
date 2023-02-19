use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    mem::size_of,
};

use anyhow::{bail, Context, Error};
use clap::Args;
use dacti_pack::{
    IndexComponentHeader, IndexEntry, IndexGroup, IndexGroupEncoding, INDEX_COMPONENT_UUID,
};
use daicon::{ComponentEntry, ComponentTableHeader, RegionData};
use tracing::{event, Level};
use uuid::uuid;

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
    event!(Level::INFO, "adding file to package...");

    // The first 32kb is reserved for components and indices
    let data_start = 1024 * 32;

    // Open the target package
    let mut package = OpenOptions::new()
        .read(true)
        .write(true)
        .open(command.package)
        .context("failed to open target package for writing")?;

    // TODO: Find a free slot rather than just assuming there's no files yet
    // TODO: Update the index table
    let data = std::fs::read(&command.file)?;

    // Find the current location of the index component
    let (table, entry_i) = find_index_component(&mut package)?;
    let region = RegionData::from_bytes(entry_i.value.data());
    let region_offset = table.region_offset() + region.offset() as u64;

    // Add entries for the new file's location and size
    let mut header = IndexComponentHeader::new();
    package.seek(SeekFrom::Start(region_offset))?;
    package.read_exact(header.as_bytes_mut())?;
    header.set_regions(1);
    package.seek(SeekFrom::Start(region_offset))?;
    package.write_all(header.as_bytes())?;

    let mut group = IndexGroup::new();
    group.set_encoding(IndexGroupEncoding::None);
    group.set_length(1);
    package.write_all(group.as_bytes())?;

    let mut entry = IndexEntry::new();
    entry.set_uuid(uuid!("bacc2ba1-8dc7-4d54-a7a4-cdad4d893a1b"));
    entry.set_offset(data_start as u32);
    entry.set_size(data.len() as u32);
    package.write_all(entry.as_bytes())?;

    // Write the file to the package
    package.seek(SeekFrom::Start(data_start))?;
    package.write_all(&data)?;

    Ok(())
}

fn find_index_component(
    package: &mut File,
) -> Result<(ComponentTableHeader, Indexed<ComponentEntry>), Error> {
    let mut header = ComponentTableHeader::new();
    package.seek(SeekFrom::Start(8))?;
    package.read_exact(header.as_bytes_mut())?;

    // TODO: Follow extensions

    let mut entry_offset = package.stream_position()?;
    for _ in 0..header.length() {
        let mut entry = ComponentEntry::new();
        package.read_exact(entry.as_bytes_mut())?;

        // Continue until we find the correct component
        if entry.type_uuid() != INDEX_COMPONENT_UUID {
            entry_offset = package.seek(SeekFrom::Current(size_of::<ComponentEntry>() as i64))?;
            continue;
        }

        let entry_i = Indexed {
            offset: entry_offset,
            value: entry,
        };
        return Ok((header, entry_i));
    }

    bail!("unable to find index component");
}

/// Combination value and its index as byte offset.
struct Indexed<T> {
    #[allow(dead_code)]
    offset: u64,
    value: T,
}
