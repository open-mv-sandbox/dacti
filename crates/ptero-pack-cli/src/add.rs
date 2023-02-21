use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    sync::Mutex,
};

use anyhow::{anyhow, Context as ContextExt, Error};
use clap::Args;
use ptero_pack::{io::RwMessage, package_add_data};
use stewart::{handler::Handler, ActorOps, Address};
use stewart_messages::StartActor;
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

    let input = std::fs::read(&command.input)?;

    // Set up the runtime
    let runtime = Runtime::new();
    let start_addr = runtime.start_actor_manager();

    // Add the package IO handler
    runtime.send(
        start_addr,
        create_file_actor(command.package, start_addr, input, command.uuid),
    );

    // TODO: We should have some way to move the add task back to here

    // Run until we're done
    runtime.block_execute();

    // TODO: Stewart doesn't currently bubble up errors for us to catch, and we need those for the
    // correct error code.

    Ok(())
}

fn create_file_actor(
    path: String,
    start_addr: Address<StartActor>,
    input: Vec<u8>,
    uuid: Uuid,
) -> StartActor {
    StartActor::new(move |ops| {
        let package = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .context("failed to open target package for writing")?;
        let package_actor = FileRwHandler {
            file: Mutex::new(package),
        };
        let package_addr = ops.add_handler(package_actor);

        // Start the add task
        let msg = StartActor::new(move |ops| package_add_data(ops, package_addr, input, uuid));
        ops.send(start_addr, msg);

        Ok(())
    })
}

struct FileRwHandler {
    file: Mutex<File>,
}

impl Handler for FileRwHandler {
    type Message = RwMessage;

    fn handle(&self, ops: &dyn ActorOps, message: RwMessage) -> Result<(), Error> {
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

        Ok(())
    }
}
