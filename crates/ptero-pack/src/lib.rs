//! Pterodactil Bring-Your-Own-IO dacti reading and writing library.

mod component;
pub mod io;

use std::io::Write;

use anyhow::Error;
use dacti_pack::{
    IndexComponentHeader, IndexEntry, IndexGroupEncoding, IndexGroupHeader, INDEX_COMPONENT_UUID,
};
use daicon::RegionData;
use stewart::{handler::Handler, ActorOps, Address};
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
    component::{find_component, FindComponentResult},
    io::RwMessage,
};

pub fn package_add_data(
    ops: &dyn ActorOps,
    package_addr: Address<RwMessage>,
    data: Vec<u8>,
    uuid: Uuid,
) {
    event!(Level::DEBUG, "adding data to package");

    // The first 64kb is reserved for components and indices
    let data_start = 1024 * 64;
    let data_len = data.len() as u32;

    // Add the index for the file to the package
    let mut index_entry = IndexEntry::zeroed();
    index_entry.set_uuid(uuid);
    index_entry.set_offset(data_start as u32);
    index_entry.set_size(data_len);
    add_index_entry(ops, package_addr, index_entry);

    // Write the file to the package
    let msg = RwMessage::Write {
        start: data_start,
        data,
    };
    ops.send(package_addr, msg);
}

fn add_index_entry(ops: &dyn ActorOps, package_addr: Address<RwMessage>, value: IndexEntry) {
    FindComponentStep::start(ops, package_addr, value);
}

struct FindComponentStep {
    package_addr: Address<RwMessage>,
    value: IndexEntry,
}

impl FindComponentStep {
    fn start(ops: &dyn ActorOps, package_addr: Address<RwMessage>, value: IndexEntry) {
        find_component(
            ops,
            INDEX_COMPONENT_UUID,
            package_addr,
            ops.add_handler(Self {
                package_addr,
                value,
            }),
        );
    }
}

impl Handler for FindComponentStep {
    type Message = FindComponentResult;

    fn handle(&self, ops: &dyn ActorOps, message: FindComponentResult) -> Result<(), Error> {
        let (_component_location, table_header, component_entry) = message?;

        let region = RegionData::from_bytes(component_entry.data());
        let component_offset = table_header.entries_offset() + region.offset() as u64;

        // TODO: Find a free slot rather than just assuming there's no groups and files yet
        // TODO: Update the component's size

        // Write the new table
        let data = create_table_data(&self.value)?;
        let msg = RwMessage::Write {
            start: component_offset,
            data,
        };
        ops.send(self.package_addr, msg);

        // TODO: Clean up handler after completion
        Ok(())
    }
}

fn create_table_data(entry: &IndexEntry) -> Result<Vec<u8>, Error> {
    let mut data = Vec::new();

    // Find the current location of the index component
    let mut header = IndexComponentHeader::zeroed();
    header.set_groups(1);
    data.write_all(header.as_bytes())?;

    let mut group = IndexGroupHeader::zeroed();
    group.set_encoding(IndexGroupEncoding::None);
    group.set_length(1);
    data.write_all(group.as_bytes())?;

    data.write_all(entry.as_bytes())?;

    Ok(data)
}
