use anyhow::Error;
use clap::Args;
use ptero_pack::{io::PackageIo, AddDataActor};
use stewart::{local::StartActor, Actor, Next, Sender};
use tracing::{event, Level};
use uuid::Uuid;

use crate::io::PackageIoActor;

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

pub struct AddCommandActor {
    start: Sender<StartActor>,
    input: Vec<u8>,
    uuid: Uuid,
}

impl AddCommandActor {
    pub fn msg(start: Sender<StartActor>, command: AddCommand) -> StartActor {
        StartActor::new(move |addr| {
            event!(Level::INFO, "adding file to package");

            let input = std::fs::read(&command.input)?;

            let msg = PackageIoActor::msg(command.package, addr);
            start.send(msg);

            Ok(AddCommandActor {
                start,
                input,
                uuid: command.uuid,
            })
        })
    }
}

impl Actor for AddCommandActor {
    type Message = Sender<PackageIo>;

    fn handle(&mut self, message: Sender<PackageIo>) -> Result<Next, Error> {
        let package = message;

        let (input, uuid) = (self.input.clone(), self.uuid);
        let msg = AddDataActor::msg(self.start.clone(), package, input, uuid);
        self.start.send(msg);

        Ok(Next::Stop)
    }
}
