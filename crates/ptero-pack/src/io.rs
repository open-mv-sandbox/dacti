use std::fs::File;

use anyhow::Error;
use stewart::Address;

pub enum RwMessage {
    ReadExact {
        start: u64,
        length: u64,
        reply: Address<ReadResult>,
    },
    Write {
        start: u64,
        data: Vec<u8>,
    },
    /// Placeholder message, will be removed once transition to messages is done
    RunOnFile {
        callback: Box<dyn FnOnce(&mut File) -> Result<(), Error> + Send>,
    },
}

// TODO: Figure out a better way than passing small vectors
pub type ReadResult = Result<Vec<u8>, Error>;
