use std::{fs::OpenOptions, io::Write, path::Path};

use anyhow::{Context, Error};
use clap::Args;
use daicon::{InterfaceEntry, InterfaceTableHeader, Version};
use uuid::uuid;

#[derive(Args, Debug)]
pub struct CreateCommand {
    #[arg(short, long, value_name = "PATH")]
    package: String,
}

pub fn run(command: CreateCommand) -> Result<(), Error> {
    let package = Path::new(&command.package);

    // This tool will in the future contain command line options for altering packages, but for
    // now is just a hardcoded test tool.

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(package)
        .context("failed to open target package for writing")?;

    // Write the signature
    file.write_all(&daicon::SIGNATURE)?;

    // Write the interface table
    let mut header = InterfaceTableHeader::new();
    header.set_count(1);
    file.write_all(header.as_bytes())?;

    let entry = InterfaceEntry::new(
        uuid!("2c5e4717-b715-429b-85cd-d320d242547a"),
        Version::new(0, 0),
    );
    file.write_all(entry.as_bytes())?;

    Ok(())
}
