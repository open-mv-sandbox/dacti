//! Pterodactil implementation of the "Daicon" format.

pub mod io;

use std::{
    io::{Cursor, Read},
    mem::size_of,
};

use anyhow::{bail, Error};
use daicon::{ComponentEntry, ComponentTableHeader, SIGNATURE};
use io::ReadResult;
use stewart::{local::Factory, Actor, Next, Sender};
use uuid::Uuid;

use crate::io::ReadWrite;

#[derive(Factory)]
#[factory(FindComponent::start)]
pub struct StartFindComponent {
    pub start: Sender<Box<dyn Factory>>,
    pub target: Uuid,
    pub package: Sender<ReadWrite>,
    pub reply: Sender<FindComponentResult>,
}

pub struct FindComponentResult {
    pub header: ComponentTableHeader,
    pub entry: ComponentEntry,
}

struct FindComponent {
    sender: Sender<FindComponentMessage>,
    data: StartFindComponent,
}

impl FindComponent {
    fn start(
        sender: Sender<FindComponentMessage>,
        data: StartFindComponent,
    ) -> Result<FindComponent, Error> {
        // Start reading the header
        let read_header = StartReadHeader {
            package: data.package.clone(),
            reply: sender.clone(),
        };
        data.start.send(Box::new(read_header));

        Ok(FindComponent { sender, data })
    }
}

impl Actor for FindComponent {
    type Message = FindComponentMessage;

    fn handle(&mut self, message: FindComponentMessage) -> Result<Next, Error> {
        let next = match message {
            FindComponentMessage::Header(location, header) => {
                let read_entries = StartReadEntries {
                    package: self.data.package.clone(),
                    header_location: location,
                    header,
                    reply: self.sender.clone(),
                };
                self.data.start.send(Box::new(read_entries));

                // TODO: Follow extensions

                Next::Continue
            }
            FindComponentMessage::Entries(header, entries) => {
                if let Some(entry) = entries
                    .into_iter()
                    .find(|e| e.type_id() == self.data.target)
                {
                    let result = FindComponentResult { header, entry };
                    self.data.reply.send(result);
                } else {
                    // TODO: Better error reporting
                    bail!("unable to find component");
                }

                Next::Stop
            }
        };

        Ok(next)
    }
}

enum FindComponentMessage {
    Header(u64, ComponentTableHeader),
    Entries(ComponentTableHeader, Vec<ComponentEntry>),
}

#[derive(Factory)]
#[factory(ReadHeader::start)]
struct StartReadHeader {
    package: Sender<ReadWrite>,
    reply: Sender<FindComponentMessage>,
}

struct ReadHeader {
    reply: Sender<FindComponentMessage>,
}

impl ReadHeader {
    fn start(sender: Sender<ReadResult>, data: StartReadHeader) -> Result<Self, Error> {
        let msg = ReadWrite::Read {
            start: 0,
            length: (SIGNATURE.len() + size_of::<ComponentTableHeader>()) as u64,
            reply: sender,
        };
        data.package.send(msg);

        Ok(ReadHeader { reply: data.reply })
    }
}

impl Actor for ReadHeader {
    type Message = ReadResult;

    fn handle(&mut self, message: ReadResult) -> Result<Next, Error> {
        let data = message?;

        // Validate signature
        if &data[0..8] != SIGNATURE {
            bail!("invalid package signature");
        }

        // Read the header data
        let header_location = 8;
        let header = ComponentTableHeader::from_bytes(&data[8..]).clone();

        let msg = FindComponentMessage::Header(header_location, header);
        self.reply.send(msg);

        Ok(Next::Stop)
    }
}

#[derive(Factory)]
#[factory(ReadEntries::start)]
struct StartReadEntries {
    package: Sender<ReadWrite>,
    header_location: u64,
    header: ComponentTableHeader,
    reply: Sender<FindComponentMessage>,
}

struct ReadEntries {
    header: ComponentTableHeader,
    reply: Sender<FindComponentMessage>,
}

impl ReadEntries {
    fn start(sender: Sender<ReadResult>, data: StartReadEntries) -> Result<Self, Error> {
        let msg = ReadWrite::Read {
            start: data.header_location + ComponentTableHeader::bytes_len() as u64,
            length: (data.header.length() as usize * size_of::<ComponentEntry>()) as u64,
            reply: sender,
        };
        data.package.send(msg);

        Ok(ReadEntries {
            header: data.header,
            reply: data.reply,
        })
    }
}

impl Actor for ReadEntries {
    type Message = ReadResult;

    fn handle(&mut self, message: ReadResult) -> Result<Next, Error> {
        let data = message?;

        let mut entries = Vec::new();
        let mut data = Cursor::new(data);

        // TODO: Direct cast?
        for _ in 0..self.header.length() {
            let mut entry = ComponentEntry::zeroed();
            data.read_exact(&mut entry)?;
            entries.push(entry);
        }

        // Reply with the read data
        let msg = FindComponentMessage::Entries(self.header.clone(), entries);
        self.reply.send(msg);

        Ok(Next::Stop)
    }
}
