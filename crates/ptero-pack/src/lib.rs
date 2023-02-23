//! Pterodactil Bring-Your-Own-IO dacti reading and writing library.

mod component;
pub mod io;

use std::io::Write;

use anyhow::Error;
use component::FindComponentResult;
use dacti_pack::{
    IndexComponentHeader, IndexEntry, IndexGroupEncoding, IndexGroupHeader, INDEX_COMPONENT_UUID,
};
use daicon::data::RegionData;
use stewart::{Actor, Next, Sender, local::StartActor};
use tracing::{event, Level};
use uuid::Uuid;

use crate::{component::FindComponentActor, io::PackageIo};

pub struct AddDataActor;

impl AddDataActor {
    pub fn msg(
        start: Sender<StartActor>,
        package: Sender<PackageIo>,
        data: Vec<u8>,
        uuid: Uuid,
    ) -> StartActor {
        StartActor::new(move |_sender| {
            event!(Level::DEBUG, "adding data to package");

            // The first 64kb is reserved for components and indices
            // TODO: Actually find a free spot
            let data_start = 1024 * 64;
            let data_len = data.len() as u32;

            // Add the index for the file to the package
            let mut index_entry = IndexEntry::zeroed();
            index_entry.set_uuid(uuid);
            index_entry.set_offset(data_start as u32);
            index_entry.set_size(data_len);
            let msg = AddIndexActor::msg(start.clone(), package.clone(), index_entry);
            start.send(msg);

            // Write the file to the package
            let msg = PackageIo::Write {
                start: data_start,
                data,
            };
            package.send(msg);

            Ok(AddDataActor)
        })
    }
}

impl Actor for AddDataActor {
    type Message = ();

    fn handle(&mut self, _message: ()) -> Result<Next, Error> {
        // TODO: Report success/failure back
        unimplemented!()
    }
}

struct AddIndexActor {
    package: Sender<PackageIo>,
    value: IndexEntry,
}

impl AddIndexActor {
    pub fn msg(
        start: Sender<StartActor>,
        package: Sender<PackageIo>,
        value: IndexEntry,
    ) -> StartActor {
        StartActor::new(move |addr| {
            let msg =
                FindComponentActor::msg(start.clone(), INDEX_COMPONENT_UUID, package.clone(), addr);
            start.send(msg);

            Ok(Self { package, value })
        })
    }
}

impl Actor for AddIndexActor {
    type Message = FindComponentResult;

    fn handle(&mut self, message: FindComponentResult) -> Result<Next, Error> {
        let region = RegionData::from_bytes(message.entry.data());
        let component_offset = region.offset(message.header.entries_offset());

        // TODO: Find a free slot rather than just assuming there's no groups and files yet
        // TODO: Update the component's size after adding the new index

        // Write the new table
        let data = create_table_data(&self.value)?;
        let msg = PackageIo::Write {
            start: component_offset,
            data,
        };
        self.package.send(msg);

        Ok(Next::Stop)
    }
}

fn create_table_data(entry: &IndexEntry) -> Result<Vec<u8>, Error> {
    let mut data = Vec::new();

    // Find the current location of the index component
    let mut header = IndexComponentHeader::zeroed();
    header.set_groups(1);
    data.write_all(&header)?;

    let mut group = IndexGroupHeader::zeroed();
    group.set_encoding(IndexGroupEncoding::None);
    group.set_length(1);
    data.write_all(&group)?;

    data.write_all(&entry)?;

    Ok(data)
}
