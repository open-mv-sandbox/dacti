use std::{
    io::{Cursor, Read},
    mem::size_of,
    sync::Arc,
};

use anyhow::{bail, Error};
use daicon::{ComponentEntry, ComponentTableHeader, SIGNATURE};
use stewart::{handler::Handler, ActorOps, Address};
use uuid::Uuid;

use crate::io::{ReadResult, RwMessage};

pub fn find_component(
    ops: &dyn ActorOps,
    target: Uuid,
    package_addr: Address<RwMessage>,
    reply: Address<FindComponentResult>,
) {
    let data = FindComponentData {
        target,
        package_addr,
        reply,
    };
    ReadHeaderStep::start(ops, Arc::new(data));
}

/// address of entry, header, entry
pub type FindComponentResult = Result<(u64, ComponentTableHeader, ComponentEntry), Error>;

struct FindComponentData {
    target: Uuid,
    package_addr: Address<RwMessage>,
    reply: Address<FindComponentResult>,
}

struct ReadHeaderStep {
    task: Arc<FindComponentData>,
}

impl ReadHeaderStep {
    fn start(ops: &dyn ActorOps, task: Arc<FindComponentData>) {
        let package_addr = task.package_addr;
        let msg = RwMessage::ReadExact {
            start: 0,
            length: (SIGNATURE.len() + size_of::<ComponentTableHeader>()) as u64,
            reply: ops.add_handler(Self { task }),
        };
        ops.send(package_addr, msg);
    }
}

impl Handler for ReadHeaderStep {
    type Message = ReadResult;

    fn handle(&self, ops: &dyn ActorOps, message: ReadResult) -> Result<(), Error> {
        let data = message?;

        // Validate signature
        if &data[0..8] != SIGNATURE {
            bail!("invalid package signature");
        }

        // Read the header data
        let header_location = 8;
        let header = ComponentTableHeader::from_bytes(&data[8..]).clone();

        // TODO: Follow extensions

        // Read the data under the table
        ReadEntriesStep::start(ops, self.task.clone(), header_location, header);

        // TODO: Clean up handler after completion
        Ok(())
    }
}

struct ReadEntriesStep {
    task: Arc<FindComponentData>,
    header: ComponentTableHeader,
}

impl ReadEntriesStep {
    fn start(
        ops: &dyn ActorOps,
        task: Arc<FindComponentData>,
        header_location: u64,
        header: ComponentTableHeader,
    ) {
        let package_addr = task.package_addr;
        let this = Self { task, header };

        let msg = RwMessage::ReadExact {
            start: header_location + size_of::<ComponentTableHeader>() as u64,
            length: (this.header.length() as usize * size_of::<ComponentEntry>()) as u64,
            reply: ops.add_handler(this),
        };
        ops.send(package_addr, msg);
    }
}

impl Handler for ReadEntriesStep {
    type Message = ReadResult;

    fn handle(&self, ops: &dyn ActorOps, message: ReadResult) -> Result<(), Error> {
        let data = message?;

        let mut entry = ComponentEntry::zeroed();
        let mut data = Cursor::new(data);

        for _ in 0..self.header.length() {
            data.read_exact(entry.as_bytes_mut())?;

            // Continue until we find the correct component
            if entry.type_uuid() != self.task.target {
                continue;
            }

            // We're done!
            let address = 8 + size_of::<ComponentTableHeader>() as u64 + data.position();
            ops.send(self.task.reply, Ok((address, self.header.clone(), entry)));

            // TODO: Clean up handler after completion
            return Ok(());
        }

        bail!("failed to find component");
    }
}