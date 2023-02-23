use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};

use anyhow::{Context, Error};
use ptero_pack::io::PackageIo;
use stewart::{local::StartActor, Actor, Next, Sender};
use tracing::{event, Level};

pub struct PackageIoActor {
    package_file: File,
}

impl PackageIoActor {
    pub fn msg(path: String, reply: Sender<Sender<PackageIo>>) -> StartActor {
        StartActor::new(move |addr| {
            let package_file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .context("failed to open target package for writing")?;

            reply.send(addr);

            Ok(Self { package_file })
        })
    }
}

impl Actor for PackageIoActor {
    type Message = PackageIo;

    fn handle(&mut self, message: PackageIo) -> Result<Next, Error> {
        match message {
            PackageIo::Read {
                start,
                length,
                reply,
            } => {
                event!(Level::TRACE, "performing read");
                let mut buffer = vec![0u8; length as usize];
                self.package_file.seek(SeekFrom::Start(start))?;
                self.package_file.read_exact(&mut buffer)?;
                reply.send(Ok(buffer));
            }
            PackageIo::Write { start, data } => {
                event!(Level::TRACE, "performing write");
                self.package_file.seek(SeekFrom::Start(start))?;
                self.package_file.write_all(&data)?;
            }
        }

        Ok(Next::Continue)
    }
}
