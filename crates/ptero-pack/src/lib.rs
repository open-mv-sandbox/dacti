//! Pterodactil Bring-Your-Own-IO dacti reading and writing library.

mod component;
pub mod io;

use std::io::Write;

use anyhow::Error;
use dacti_pack::{
    IndexComponentHeader, IndexEntry, IndexGroupEncoding, IndexGroupHeader, INDEX_COMPONENT_UUID,
};
use daicon::data::RegionData;
use stewart::{ActorOps, Address, Handler, Next};
use stewart_runtime::StartActor;
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
    component::{find_component_actor, FindComponentResult},
    io::RwMessage,
};

pub fn add_data_actor(
    start_addr: Address<StartActor>,
    package_addr: Address<RwMessage>,
    data: Vec<u8>,
    uuid: Uuid,
) -> StartActor {
    StartActor::new(move |ops| {
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
        start_add_index_entry(ops, start_addr, package_addr, index_entry);

        // Write the file to the package
        let msg = RwMessage::Write {
            start: data_start,
            data,
        };
        ops.send(package_addr, msg);

        Ok(())
    })
}

fn start_add_index_entry(
    ops: &dyn ActorOps,
    start_addr: Address<StartActor>,
    package_addr: Address<RwMessage>,
    value: IndexEntry,
) {
    let addr = ops.add_handler(FindComponentResultHandler {
        package_addr,
        value,
    });

    let msg = find_component_actor(INDEX_COMPONENT_UUID, package_addr, addr);
    ops.send(start_addr, msg);
}

struct FindComponentResultHandler {
    package_addr: Address<RwMessage>,
    value: IndexEntry,
}

impl Handler for FindComponentResultHandler {
    type Message = FindComponentResult;

    fn handle(&self, ops: &dyn ActorOps, message: FindComponentResult) -> Result<Next, Error> {
        let (_component_location, table_header, component_entry) = message?;

        let region = RegionData::from_bytes(component_entry.data());
        let component_offset = region.offset(table_header.entries_offset());

        // TODO: Find a free slot rather than just assuming there's no groups and files yet
        // TODO: Update the component's size after adding the new index

        // Write the new table
        let data = create_table_data(&self.value)?;
        let msg = RwMessage::Write {
            start: component_offset,
            data,
        };
        ops.send(self.package_addr, msg);

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
