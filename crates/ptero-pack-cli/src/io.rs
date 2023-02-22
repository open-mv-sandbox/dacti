use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    sync::Mutex,
};

use anyhow::{anyhow, Context, Error};
use ptero_pack::io::RwMessage;
use stewart::{ActorOps, Address, Handler, Next};
use stewart_api_runtime::StartActor;

pub fn file_actor(path: String, reply: Address<Address<RwMessage>>) -> StartActor {
    StartActor::new(move |ops| {
        let package = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .context("failed to open target package for writing")?;
        let package_actor = FileRwHandler {
            file: Mutex::new(package),
        };
        let rw_addr = ops.add_handler(package_actor);

        // Notify that we're ready
        ops.send(reply, rw_addr);

        Ok(())
    })
}

struct FileRwHandler {
    file: Mutex<File>,
}

impl Handler for FileRwHandler {
    type Message = RwMessage;

    fn handle(&self, ops: &dyn ActorOps, message: RwMessage) -> Result<Next, Error> {
        let mut file = self.file.lock().map_err(|_| anyhow!("lock poisoned"))?;

        match message {
            RwMessage::ReadExact {
                start,
                length,
                reply,
            } => {
                // TODO: Cache buffer
                // TODO: Non-exact streaming reads
                let mut buffer = vec![0u8; length as usize];
                file.seek(SeekFrom::Start(start))?;
                file.read_exact(&mut buffer)?;
                ops.send(reply, Ok(buffer));
            }
            RwMessage::Write { start, data } => {
                file.seek(SeekFrom::Start(start))?;
                file.write_all(&data)?;
            }
        }

        Ok(Next::Continue)
    }
}
