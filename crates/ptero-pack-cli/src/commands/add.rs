use anyhow::Error;
use clap::Args;
use ptero_daicon::io::ReadWrite;
use ptero_pack::AddData;
use stewart::{Actor, Next};
use stewart_local::{Address, Context, Factory};
use tracing::{event, Level};
use uuid::Uuid;

use crate::io::FileReadWrite;

/// Add files to a dacti package.
#[derive(Factory, Args, Debug)]
#[factory(AddCommandActor::start)]
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

struct AddCommandActor {
    ctx: Context,
    input: Vec<u8>,
    uuid: Uuid,
}

impl AddCommandActor {
    pub fn start(
        ctx: Context,
        address: Address<Address<ReadWrite>>,
        data: AddCommand,
    ) -> Result<Self, Error> {
        event!(Level::INFO, "adding file to package");

        let input = std::fs::read(&data.input)?;

        let start_file = FileReadWrite {
            path: data.package,
            reply: address,
        };
        ctx.start(start_file);

        Ok(AddCommandActor {
            ctx,
            input,
            uuid: data.uuid,
        })
    }
}

impl Actor for AddCommandActor {
    type Message = Address<ReadWrite>;

    fn handle(&mut self, message: Address<ReadWrite>) -> Result<Next, Error> {
        let package = message;

        let (input, uuid) = (self.input.clone(), self.uuid);
        let add_data = AddData {
            package,
            data: input,
            uuid,
        };
        self.ctx.start(add_data);

        Ok(Next::Stop)
    }
}
