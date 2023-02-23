//! Pterodactil implementation of the "Dacti Package" format.

use std::io::Write;

use anyhow::Error;
use dacti_pack::{
    IndexComponentHeader, IndexEntry, IndexGroupEncoding, IndexGroupHeader, INDEX_COMPONENT_UUID,
};
use daicon::data::RegionData;
use ptero_daicon::{io::ReadWrite, FindComponentResult, StartFindComponent};
use stewart::{local::Factory, Actor, Next, Sender};
use tracing::{event, Level};
use uuid::Uuid;

#[derive(Factory)]
#[factory(AddData::start)]
pub struct StartAddData {
    pub start: Sender<Box<dyn Factory>>,
    pub package: Sender<ReadWrite>,
    pub data: Vec<u8>,
    pub uuid: Uuid,
}

struct AddData;

impl AddData {
    pub fn start(_sender: Sender<()>, data: StartAddData) -> Result<Self, Error> {
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
        let add_index = StartAddIndex {
            start: data.start.clone(),
            package: data.package.clone(),
            value: index_entry,
        };
        data.start.send(Box::new(add_index));

        // Write the file to the package
        let msg = ReadWrite::Write {
            start: data_start,
            data: data.data,
        };
        data.package.send(msg);

        Ok(AddData)
    }
}

impl Actor for AddData {
    type Message = ();

    fn handle(&mut self, _message: ()) -> Result<Next, Error> {
        // TODO: Report success/failure back
        unimplemented!()
    }
}

#[derive(Factory)]
#[factory(AddIndex::start)]
struct StartAddIndex {
    start: Sender<Box<dyn Factory>>,
    package: Sender<ReadWrite>,
    value: IndexEntry,
}

struct AddIndex {
    package: Sender<ReadWrite>,
    value: IndexEntry,
}

impl AddIndex {
    pub fn start(sender: Sender<FindComponentResult>, data: StartAddIndex) -> Result<Self, Error> {
        let find_component = StartFindComponent {
            start: data.start.clone(),
            target: INDEX_COMPONENT_UUID,
            package: data.package.clone(),
            reply: sender,
        };
        data.start.send(Box::new(find_component));

        Ok(Self {
            package: data.package,
            value: data.value,
        })
    }
}

impl Actor for AddIndex {
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
