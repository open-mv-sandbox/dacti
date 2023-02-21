use anyhow::Error;
use clap::Args;
use ptero_pack::{io::RwMessage, package_add_data};
use stewart::{
    handler::{Handler, Next},
    ActorOps, Address,
};
use stewart_runtime::StartActor;
use tracing::{event, Level};
use uuid::Uuid;

use crate::io::file_actor;

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

pub fn actor(command: AddCommand, start_addr: Address<StartActor>) -> StartActor {
    StartActor::new(move |opt| {
        event!(Level::INFO, "adding file to package...");

        let input = std::fs::read(&command.input)?;

        let ready_addr = opt.add_handler(ReadyHandler {
            start_addr,
            input,
            uuid: command.uuid,
        });

        opt.send(start_addr, file_actor(command.package, ready_addr));

        Ok(())
    })
}

struct ReadyHandler {
    start_addr: Address<StartActor>,
    input: Vec<u8>,
    uuid: Uuid,
}

impl Handler for ReadyHandler {
    type Message = Address<RwMessage>;

    fn handle(&self, ops: &dyn ActorOps, message: Self::Message) -> Result<Next, Error> {
        let package_addr = message;

        // TODO: Could we do a once-handler that takes by value?
        let (input, uuid) = (self.input.clone(), self.uuid);
        let msg = StartActor::new(move |ops| package_add_data(ops, package_addr, input, uuid));
        ops.send(self.start_addr, msg);

        Ok(Next::Stop)
    }
}
