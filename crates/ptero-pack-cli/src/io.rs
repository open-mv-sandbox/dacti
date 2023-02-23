use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};

use anyhow::{Context, Error};
use ptero_daicon::io::ReadWrite;
use stewart::{local::Factory, Actor, Next, Sender};
use tracing::{event, Level};

#[derive(Factory)]
#[factory(FileReadWrite::start)]
pub struct StartFileReadWrite {
    pub path: String,
    pub reply: Sender<Sender<ReadWrite>>,
}

struct FileReadWrite {
    package_file: File,
}

impl FileReadWrite {
    pub fn start(sender: Sender<ReadWrite>, data: StartFileReadWrite) -> Result<Self, Error> {
        let package_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(data.path)
            .context("failed to open target package for writing")?;

        data.reply.send(sender);

        Ok(Self { package_file })
    }
}

impl Actor for FileReadWrite {
    type Message = ReadWrite;

    fn handle(&mut self, message: ReadWrite) -> Result<Next, Error> {
        match message {
            ReadWrite::Read {
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
            ReadWrite::Write { start, data } => {
                event!(Level::TRACE, "performing write");
                self.package_file.seek(SeekFrom::Start(start))?;
                self.package_file.write_all(&data)?;
            }
        }

        Ok(Next::Continue)
    }
}
