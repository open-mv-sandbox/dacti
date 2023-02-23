use std::{
    io::{Cursor, Read},
    mem::size_of,
};

use anyhow::{bail, Error};
use daicon::{ComponentEntry, ComponentTableHeader, SIGNATURE};
use stewart::{local::StartActor, Actor, Next, Sender};
use uuid::Uuid;

use crate::io::{PackageIo, ReadResult};

pub struct FindComponentActor {
    sender: Sender<FindComponentMessage>,
    start: Sender<StartActor>,
    package: Sender<PackageIo>,
    target: Uuid,
    reply: Sender<FindComponentResult>,
}

impl FindComponentActor {
    // TODO: Private actor type should not be exposed
    pub fn msg(
        start: Sender<StartActor>,
        target: Uuid,
        package: Sender<PackageIo>,
        reply: Sender<FindComponentResult>,
    ) -> StartActor {
        StartActor::new(move |sender| {
            // Start reading the header
            let msg = ReadHeaderActor::msg(package.clone(), sender.clone());
            start.send(msg);

            Ok(Self {
                sender,
                start,
                package,
                target,
                reply,
            })
        })
    }
}

impl Actor for FindComponentActor {
    type Message = FindComponentMessage;

    fn handle(&mut self, message: FindComponentMessage) -> Result<Next, Error> {
        let next = match message {
            FindComponentMessage::Header(location, header) => {
                let msg = ReadEntriesActor::msg(
                    self.package.clone(),
                    location,
                    header,
                    self.sender.clone(),
                );
                self.start.send(msg);

                // TODO: Follow extensions

                Next::Continue
            }
            FindComponentMessage::Entries(header, entries) => {
                if let Some(entry) = entries.into_iter().find(|e| e.type_id() == self.target) {
                    let result = FindComponentResult { header, entry };
                    self.reply.send(result);
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

pub struct FindComponentResult {
    pub header: ComponentTableHeader,
    pub entry: ComponentEntry,
}

pub enum FindComponentMessage {
    Header(u64, ComponentTableHeader),
    Entries(ComponentTableHeader, Vec<ComponentEntry>),
}

struct ReadHeaderActor {
    reply: Sender<FindComponentMessage>,
}

impl ReadHeaderActor {
    fn msg(package: Sender<PackageIo>, reply: Sender<FindComponentMessage>) -> StartActor {
        StartActor::new(move |addr| {
            let msg = PackageIo::Read {
                start: 0,
                length: (SIGNATURE.len() + size_of::<ComponentTableHeader>()) as u64,
                reply: addr,
            };
            package.send(msg);

            Ok(Self { reply })
        })
    }
}

impl Actor for ReadHeaderActor {
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

struct ReadEntriesActor {
    header: ComponentTableHeader,
    reply: Sender<FindComponentMessage>,
}

impl ReadEntriesActor {
    fn msg(
        package: Sender<PackageIo>,
        header_location: u64,
        header: ComponentTableHeader,
        reply: Sender<FindComponentMessage>,
    ) -> StartActor {
        StartActor::new(move |addr| {
            let msg = PackageIo::Read {
                start: header_location + ComponentTableHeader::bytes_len() as u64,
                length: (header.length() as usize * size_of::<ComponentEntry>()) as u64,
                reply: addr,
            };
            package.send(msg);

            Ok(Self { header, reply })
        })
    }
}

impl Actor for ReadEntriesActor {
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
