use std::io::Write;

use anyhow::Error;
use bytemuck::{bytes_of, Pod, Zeroable};
use uuid::Uuid;

use crate::{Version, SIGNATURE};

#[repr(transparent)]
pub struct FormatHeader(FormatHeaderRaw);

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
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

    pub fn set_type_uuid(&mut self, value: Uuid) {
        self.0.type_uuid = value.to_bytes_le();
    }

    pub fn version(&self) -> Version {
        Version {
            major: u16::from_le(self.0.version_major),
            minor: u16::from_le(self.0.version_minor),
        }
    }

    pub fn set_version(&mut self, value: Version) {
        self.0.version_major = value.major.to_le();
        self.0.version_minor = value.minor.to_le();
    }

    pub fn write_with_signature_to<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&SIGNATURE)?;
        writer.write_all(bytes_of(&self.0))?;
        Ok(())
    }
}
