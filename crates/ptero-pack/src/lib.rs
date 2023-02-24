//! Pterodactil implementation of the "Dacti Package" format.

use std::io::Write;

use anyhow::Error;
use dacti_pack::{
    IndexComponentHeader, IndexEntry, IndexGroupEncoding, IndexGroupHeader, INDEX_COMPONENT_UUID,
};
use daicon::data::RegionData;
use ptero_daicon::{io::ReadWrite, FindComponent, FindComponentResult};
use stewart::{Actor, Next};
use stewart_local::{Address, Context, Factory};
use tracing::{event, Level};
use uuid::Uuid;

#[derive(Factory)]
#[factory(AddDataActor::start)]
pub struct AddData {
    pub package: Address<ReadWrite>,
    pub data: Vec<u8>,
    pub uuid: Uuid,
}

struct AddDataActor;

impl AddDataActor {
    pub fn start(ctx: Context, _address: Address<()>, data: AddData) -> Result<Self, Error> {
        event!(Level::DEBUG, "adding data to package");

        // The first 64kb is reserved for components and indices
        // TODO: Actually find a free spot
        let data_start = 1024 * 64;
        let data_len = data.data.len() as u32;

        // Add the index for the file to the package
        let mut index_entry = IndexEntry::zeroed();
        index_entry.set_uuid(data.uuid);
        index_entry.set_offset(data_start as u32);
        index_entry.set_size(data_len);
        let add_index = AddIndex {
            package: data.package.clone(),
            value: index_entry,
        };
        ctx.start(add_index);

        // Write the file to the package
        let write = ReadWrite::Write {
            start: data_start,
            data: data.data,
        };
        ctx.send(data.package, write);

        Ok(AddDataActor)
    }
}

impl Actor for AddDataActor {
    type Message = ();

    fn handle(&mut self, _message: ()) -> Result<Next, Error> {
        // TODO: Report success/failure back
        unimplemented!()
    }
}

#[derive(Factory)]
#[factory(AddIndexActor::start)]
struct AddIndex {
    package: Address<ReadWrite>,
    value: IndexEntry,
}

struct AddIndexActor {
    ctx: Context,
    package: Address<ReadWrite>,
    value: IndexEntry,
}

impl AddIndexActor {
    pub fn start(
        ctx: Context,
        address: Address<FindComponentResult>,
        data: AddIndex,
    ) -> Result<Self, Error> {
        let find_component = FindComponent {
            target: INDEX_COMPONENT_UUID,
            package: data.package.clone(),
            reply: address,
        };
        ctx.start(find_component);

        Ok(Self {
            ctx,
            package: data.package,
            value: data.value,
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
        let msg = ReadWrite::Write {
            start: component_offset,
            data,
        };
        self.ctx.send(self.package, msg);

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
