use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    sync::Mutex,
};

use anyhow::{anyhow, Context as ContextExt, Error};
use clap::Args;
use ptero_pack::{io::RwMessage, package_add_data};
use stewart::{Actor, Context};
use stewart_native::Runtime;
use tracing::{event, Level};
use uuid::Uuid;

/// Add files to a dacti package.
#[derive(Args, Debug)]
pub struct AddCommand {
    /// The path of the package to add files to.
    #[arg(short, long, value_name = "PATH")]
    package: String,

    /// The input location of the file to add.
    #[arg(short, long, value_name = "PATH")]
    input: String,

    /// The UUID to assign the input file.
    #[arg(short, long, value_name = "UUID")]
    uuid: Uuid,
}

pub fn run(command: AddCommand) -> Result<(), Error> {
    event!(Level::INFO, "adding file to package...");

    let package = OpenOptions::new()
        .read(true)
        .write(true)
        .open(command.package)
        .context("failed to open target package for writing")?;
    let input = std::fs::read(&command.input)?;

    // Set up the runtime
    let runtime = Runtime::new();
    let ctx = runtime.context().clone();

    // Add the package IO handler
    let package_actor = FileRwHandler {
        file: Mutex::new(package),
    };
    let package_addr = ctx.add_actor(package_actor);

    // Start the add task
    package_add_data(&ctx, package_addr, input, command.uuid);

    // Run until we're done
    runtime.block_execute();

    // TODO: Stewart doesn't currently bubble up errors for us to catch, and we need those for the
    // correct error code.

    Ok(())
}

struct FileRwHandler {
    file: Mutex<File>,
}

impl Actor for FileRwHandler {
    type Message = RwMessage;

    fn handle(&self, ctx: &Context, message: RwMessage) -> Result<(), Error> {
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
                ctx.send(reply, Ok(buffer));
            }
            RwMessage::Write { start, data } => {
                file.seek(SeekFrom::Start(start))?;
                file.write_all(&data)?;
            }
            RwMessage::RunOnFile { callback } => {
                callback(&mut file)?;
            }
        }

        Ok(())
    }
}
