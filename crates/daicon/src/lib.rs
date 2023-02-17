//! Daicon writing and reading utility library.
//!
//! This library does not guarantee 100% correctness in input or output, but will provide minimal
//! validation where useful. In most cases, you should not use this library directly, but instead
//! use a format-specific library that uses this library.
//!
//! This library does maintain endianness compatibility, while maintaing near-native performance
//! on little-endian systems in release build mode.
//!
//! Where possible, high-level types are `#[repr(transparent)]` to the low-level data of those
//! types, and can be safely reinterpreted. Internally, this library uses bytemuck, but this is
//! not exposed publically.

use std::io::{Seek, SeekFrom, Write};

use anyhow::Error;
use bytemuck::{bytes_of, Pod, Zeroable};
use uuid::Uuid;

pub const SIGNATURE: &[u8] = b"\xFFdaicon0";

#[repr(transparent)]
pub struct FormatHeader(FormatHeaderRaw);

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct FormatHeaderRaw {
    type_uuid: [u8; 16],
    version_major: u16,
    version_minor: u16,
}

impl FormatHeader {
    pub fn new(type_uuid: Uuid, version: Version) -> Self {
        let mut value = Self(Zeroable::zeroed());
        value.set_type_uuid(type_uuid);
        value.set_version(version);
        value
    }

    pub fn type_uuid(&self) -> Uuid {
        Uuid::from_bytes_le(self.0.type_uuid)
    }

    pub fn set_type_uuid(&mut self, type_uuid: Uuid) {
        self.0.type_uuid = type_uuid.to_bytes_le();
    }

    pub fn version(&self) -> Version {
        Version {
            major: u16::from_le(self.0.version_major),
            minor: u16::from_le(self.0.version_minor),
        }
    }

    pub fn set_version(&mut self, version: Version) {
        self.0.version_major = version.major.to_le();
        self.0.version_minor = version.minor.to_le();
    }

    pub fn write_with_signature<W: Seek + Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.seek(SeekFrom::Start(0))?;
        writer.write_all(&SIGNATURE)?;
        writer.write_all(bytes_of(&self.0))?;

        Ok(())
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}

impl Version {
    pub fn new(major: u16, minor: u16) -> Self {
        Version { major, minor }
    }
}
