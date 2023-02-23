use anyhow::Error;
use clap::Args;
use ptero_pack::{io::PackageIo, AddDataActor};
use stewart::{Actor, Next};
use stewart_local::{Address, DispatcherArc, StartActor};
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
    dispatcher: DispatcherArc,
    start_addr: Address<StartActor>,
    input: Vec<u8>,
    uuid: Uuid,
}

impl AddCommandActor {
    pub fn msg(
        dispatcher: DispatcherArc,
        start_addr: Address<StartActor>,
        command: AddCommand,
    ) -> StartActor {
        StartActor::new(move |addr| {
            event!(Level::INFO, "adding file to package");

            let input = std::fs::read(&command.input)?;

            let msg = PackageIoActor::msg(dispatcher.clone(), command.package, addr);
            dispatcher.send(start_addr, msg);

            Ok(AddCommandActor {
                dispatcher,
                start_addr,
                input,
                uuid: command.uuid,
            })
        })
    }
}

impl Actor for AddCommandActor {
    type Message = Address<PackageIo>;

    fn handle(&mut self, message: Address<PackageIo>) -> Result<Next, Error> {
        let package_addr = message;

        let (input, uuid) = (self.input.clone(), self.uuid);
        let msg = AddDataActor::msg(
            self.dispatcher.clone(),
            self.start_addr,
            package_addr,
            input,
            uuid,
        );
        self.dispatcher.send(self.start_addr, msg);

        Ok(Next::Stop)
    }
}
