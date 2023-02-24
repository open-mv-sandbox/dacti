use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};

use anyhow::{Context as ContextExt, Error};
use ptero_daicon::io::ReadWrite;
use stewart::{Actor, Next};
use stewart_local::{Address, Context, Factory};
use tracing::{event, Level};

#[derive(Factory)]
#[factory(FileReadWriteActor::start)]
pub struct FileReadWrite {
    pub path: String,
    pub reply: Address<Address<ReadWrite>>,
}

struct FileReadWriteActor {
    ctx: Context,
    package_file: File,
}

impl FileReadWriteActor {
    pub fn start(
        ctx: Context,
        address: Address<ReadWrite>,
        data: FileReadWrite,
    ) -> Result<Self, Error> {
        let package_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(data.path)
            .context("failed to open target package for writing")?;

        ctx.send(data.reply, address);

        Ok(Self { ctx, package_file })
    }
}

impl Actor for FileReadWriteActor {
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
                self.ctx.send(reply, Ok(buffer));
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
