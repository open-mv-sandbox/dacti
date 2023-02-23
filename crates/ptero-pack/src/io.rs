use anyhow::Error;
use stewart::Sender;

pub enum PackageIo {
    Read {
        start: u64,
        length: u64,
        reply: Sender<ReadResult>,
    },
    Write {
        start: u64,
        data: Vec<u8>,
    },
}

// TODO: Figure out a better way than passing small vectors
pub type ReadResult = Result<Vec<u8>, Error>;
