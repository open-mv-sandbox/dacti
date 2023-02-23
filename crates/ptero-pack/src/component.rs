use std::{
    io::{Cursor, Read},
    mem::size_of,
};

use anyhow::{bail, Error};
use daicon::{ComponentEntry, ComponentTableHeader, SIGNATURE};
use stewart::{Actor, Next};
use stewart_local::{Address, DispatcherArc, StartActor};
use uuid::Uuid;

use crate::io::{PackageIo, ReadResult};

pub struct FindComponentActor {
    addr: Address<FindComponentMessage>,
    dispatcher: DispatcherArc,
    start_addr: Address<StartActor>,
    package_addr: Address<PackageIo>,
    target: Uuid,
    reply: Address<FindComponentResult>,
}

impl FindComponentActor {
    pub fn msg(
        dispatcher: DispatcherArc,
        start_addr: Address<StartActor>,
        target: Uuid,
        package_addr: Address<PackageIo>,
        reply: Address<FindComponentResult>,
    ) -> StartActor {
        StartActor::new(move |addr| {
            // Start reading the header
            let msg = ReadHeaderActor::msg(dispatcher.clone(), package_addr, addr);
            dispatcher.send(start_addr, msg);

            Ok(Self {
                addr,
                dispatcher,
                start_addr,
                package_addr,
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
                    self.dispatcher.clone(),
                    self.package_addr,
                    location,
                    header,
                    self.addr,
                );
                self.dispatcher.send(self.start_addr, msg);

                // TODO: Follow extensions

                Next::Continue
            }
            FindComponentMessage::Entries(header, entries) => {
                if let Some(entry) = entries.into_iter().find(|e| e.type_id() == self.target) {
                    let result = FindComponentResult { header, entry };
                    self.dispatcher.send(self.reply, result);
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
    dispatcher: DispatcherArc,
    reply: Address<FindComponentMessage>,
}

impl ReadHeaderActor {
    fn msg(
        dispatcher: DispatcherArc,
        package_addr: Address<PackageIo>,
        reply: Address<FindComponentMessage>,
    ) -> StartActor {
        StartActor::new(move |addr| {
            let msg = PackageIo::Read {
                start: 0,
                length: (SIGNATURE.len() + size_of::<ComponentTableHeader>()) as u64,
                reply: addr,
            };
            dispatcher.send(package_addr, msg);

            Ok(Self { dispatcher, reply })
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
        self.dispatcher.send(self.reply, msg);

        Ok(Next::Stop)
    }
}

struct ReadEntriesActor {
    dispatcher: DispatcherArc,
    header: ComponentTableHeader,
    reply: Address<FindComponentMessage>,
}

impl ReadEntriesActor {
    fn msg(
        dispatcher: DispatcherArc,
        package_addr: Address<PackageIo>,
        header_location: u64,
        header: ComponentTableHeader,
        reply: Address<FindComponentMessage>,
    ) -> StartActor {
        StartActor::new(move |addr| {
            let msg = PackageIo::Read {
                start: header_location + ComponentTableHeader::bytes_len() as u64,
                length: (header.length() as usize * size_of::<ComponentEntry>()) as u64,
                reply: addr,
            };
            dispatcher.send(package_addr, msg);

            Ok(Self {
                dispatcher,
                header,
                reply,
            })
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
        self.dispatcher.send(self.reply, msg);

        Ok(Next::Stop)
    }
}
