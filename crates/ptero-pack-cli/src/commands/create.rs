use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
    mem::size_of,
};

use anyhow::{Context, Error};
use clap::Args;
use dacti_pack::{IndexComponentHeader, INDEX_COMPONENT_UUID};
use daicon::{data::RegionData, ComponentEntry, ComponentTableHeader};
use stewart::{local::Factory, Actor, Sender};
use tracing::{event, Level};

/// Create a new dacti package.
#[derive(Args, Debug)]
pub struct CreateCommand {
    /// The path to create the package at.
    #[arg(short, long, value_name = "PATH")]
    package: String,
}

#[derive(Factory)]
#[factory(CreateCommandActor::start)]
pub struct StartCreateCommand {
    pub command: CreateCommand,
}

struct CreateCommandActor;

impl CreateCommandActor {
    pub fn start(_sender: Sender<()>, data: StartCreateCommand) -> Result<Self, Error> {
        event!(Level::INFO, "creating package");

        // Reserve 1kb for header and component table
        let indices_offset: u32 = 1024;

        // Open the target file, overwriting anything already there
        let mut package = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(data.command.package)
            .context("failed to open target package for writing")?;

        // Write the signature
        package.write_all(daicon::SIGNATURE)?;

        // Write the component table
        let mut header = ComponentTableHeader::zeroed();
        header.set_length(1);
        package.write_all(&header)?;

        let mut entry = ComponentEntry::zeroed();
        entry.set_type_id(INDEX_COMPONENT_UUID);

        let region = RegionData::from_bytes_mut(entry.data_mut());
        region.set_relative_offset(indices_offset);
        region.set_size(size_of::<IndexComponentHeader>() as u32);

        package.write_all(&entry)?;

        // Write an empty indices table
        package.seek(SeekFrom::Start(indices_offset as u64))?;
        let header = IndexComponentHeader::zeroed();
        package.write_all(&header)?;

        Ok(CreateCommandActor)
    }
}

impl Actor for CreateCommandActor {
    type Message = ();

    fn handle(&mut self, _message: Self::Message) -> Result<stewart::Next, anyhow::Error> {
        // TODO: Currently makes no sense for this to be an actor, but it will use other actors
        unimplemented!()
    }
}
