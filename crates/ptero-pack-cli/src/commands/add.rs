use anyhow::Error;
use clap::Args;
use ptero_daicon::io::ReadWrite;
use ptero_pack::StartAddData;
use stewart::{local::Factory, Actor, Next, Sender};
use tracing::{event, Level};
use uuid::Uuid;

use crate::io::StartFileReadWrite;

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

#[derive(Factory)]
#[factory(AddCommandActor::start)]
pub struct StartAddCommand {
    pub start: Sender<Box<dyn Factory>>,
    pub command: AddCommand,
}

struct AddCommandActor {
    start: Sender<Box<dyn Factory>>,
    input: Vec<u8>,
    uuid: Uuid,
}

impl AddCommandActor {
    pub fn start(sender: Sender<Sender<ReadWrite>>, data: StartAddCommand) -> Result<Self, Error> {
        event!(Level::INFO, "adding file to package");

        let input = std::fs::read(&data.command.input)?;

        let start_file = StartFileReadWrite {
            path: data.command.package,
            reply: sender,
        };
        data.start.send(Box::new(start_file));

        Ok(AddCommandActor {
            start: data.start,
            input,
            uuid: data.command.uuid,
        })
    }
}

impl Actor for AddCommandActor {
    type Message = Sender<ReadWrite>;

    fn handle(&mut self, message: Sender<ReadWrite>) -> Result<Next, Error> {
        let package = message;

        let (input, uuid) = (self.input.clone(), self.uuid);
        let add_data = StartAddData {
            start: self.start.clone(),
            package,
            data: input,
            uuid,
        };
        self.start.send(Box::new(add_data));

        Ok(Next::Stop)
    }
}
