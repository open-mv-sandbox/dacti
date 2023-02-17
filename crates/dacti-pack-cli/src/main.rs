use std::{fs::OpenOptions, path::Path};

use anyhow::{Context, Error};
use clap::Parser;
use daicon::{FormatHeader, InterfaceEntry, InterfaceTableHeader, Version};
use uuid::uuid;

fn main() {
    let _args = Args::parse();

    // This tool will in the future contain command line options for altering packages, but for
    // now is just a hardcoded test tool.
    let result = build_pack();

    if let Err(error) = result {
        println!("failed:\n{:?}", error);
        std::process::exit(1);
    }

    println!("completed successfully");
}

/// dacti-pack CLI utility tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn build_pack() -> Result<(), Error> {
    let target = Path::new("./packages/dacti-example-web/public/viewer-builtins.dacti-pack");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(target)
        .context("failed to open target package for writing")?;

    // Write the format header
    let format = FormatHeader::new(
        uuid!("5f0f7929-7577-4be5-8bb5-4a63199b6722"),
        Version::new(0, 0),
    );
    format.write_with_signature_to(&mut file)?;

    // Write the interface table
    let mut header = InterfaceTableHeader::new();
    header.set_count(1);
    header.write_to(&mut file)?;

    let entry = InterfaceEntry::new(
        uuid!("2c5e4717-b715-429b-85cd-d320d242547a"),
        Version::new(0, 0),
    );
    entry.write_to(&mut file)?;

    Ok(())
}
