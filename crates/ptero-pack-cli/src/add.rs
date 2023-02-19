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

    // The first 64kb is reserved for components and indices
    let data_start = 1024 * 64;

    // Open the target package
    let mut package = OpenOptions::new()
        .read(true)
        .write(true)
        .open(command.package)
        .context("failed to open target package for writing")?;

    let mut input_file = File::open(&command.input)?;
    let input_size = input_file.metadata()?.len();

    add_index(
        &mut package,
        command.uuid,
        data_start as u32,
        input_size as u32,
    )?;

    // Write the file to the package
    package.seek(SeekFrom::Start(data_start))?;
    std::io::copy(&mut input_file, &mut package)?;

    Ok(())
}

fn add_index(package: &mut File, uuid: Uuid, offset: u32, size: u32) -> Result<(), Error> {
    // TODO: Find a free slot rather than just assuming there's no files yet

    // Find the current location of the index component
    let (table, entry_i) = find_component_entry(package, INDEX_COMPONENT_UUID)?;
    let region = RegionData::from_bytes(entry_i.value.data());
    let region_offset = table.region_offset() + region.offset() as u64;

    // Add entries for the new file's location and size
    get_or_add_group(package, region_offset)?;

    let mut entry = IndexEntry::new();
    entry.set_uuid(uuid);
    entry.set_offset(offset);
    entry.set_size(size);
    package.write_all(entry.as_bytes())?;

    Ok(())
}

fn get_or_add_group(package: &mut File, region_offset: u64) -> Result<(), Error> {
    // TODO: Find a free slot rather than just assuming there's no groups yet

    let mut header = IndexComponentHeader::new();
    package.seek(SeekFrom::Start(region_offset))?;
    package.read_exact(header.as_bytes_mut())?;
    header.set_groups(1);
    package.seek(SeekFrom::Start(region_offset))?;
    package.write_all(header.as_bytes())?;

    let mut group = IndexGroup::new();
    group.set_encoding(IndexGroupEncoding::None);
    group.set_length(1);
    package.write_all(group.as_bytes())?;

    Ok(())
}

fn find_component_entry(
    package: &mut File,
    uuid: Uuid,
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
        if entry.type_uuid() != uuid {
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
