use std::{fs::File, io::Write, num::NonZeroU64, path::Path};

use anyhow::Error;
use bytemuck::{bytes_of, Pod, Zeroable};
use clap::Parser;
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
    let mut file = File::create(target)?;
    file.write_all(daicon::SIGNATURE)?;

    // Write the format header
    let format = FormatHeader {
        // dacti-pack format
        type_uuid: uuid!("5f0f7929-7577-4be5-8bb5-4a63199b6722").to_bytes_le(),
        version_major: 0,
        version_minor: 0,
    };
    file.write_all(bytes_of(&format))?;

    // Write the interface table
    let header = InterfaceTableHeader {
        region_offset: 0,
        extension: None,
        reserved: 0,
        count: 1,
    };
    file.write_all(bytes_of(&header))?;

    let header = Interface {
        // dacti-pack index interface
        type_uuid: uuid!("2c5e4717-b715-429b-85cd-d320d242547a").to_bytes_le(),
        version_major: 0,
        version_minor: 0,
        data: [0, 0, 0, 0, 0, 0, 0, 0],
    };
    file.write_all(bytes_of(&header))?;

    Ok(())
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct FormatHeader {
    type_uuid: [u8; 16],
    version_major: u16,
    version_minor: u16,
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct InterfaceTableHeader {
    region_offset: u64,
    extension: Option<NonZeroU64>,
    reserved: u32,
    count: u32,
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct Interface {
    type_uuid: [u8; 16],
    version_major: u16,
    version_minor: u16,
    data: [u8; 8],
}
