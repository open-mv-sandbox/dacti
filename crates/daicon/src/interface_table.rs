use std::{io::Write, num::NonZeroU64};

use anyhow::Error;
use bytemuck::{bytes_of, Pod, Zeroable};
use uuid::Uuid;

use crate::Version;

#[repr(transparent)]
pub struct InterfaceTableHeader(InterfaceTableHeaderRaw);

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct InterfaceTableHeaderRaw {
    region_offset: u64,
    extension_offset: Option<NonZeroU64>,
    reserved: u32,
    count: u32,
}

impl InterfaceTableHeader {
    pub fn new() -> Self {
        let value = Self(Zeroable::zeroed());
        value
    }

    pub fn region_offset(&self) -> u64 {
        self.0.region_offset
    }

    pub fn set_region_offset(&mut self, value: u64) {
        self.0.region_offset = value;
    }

    pub fn extension_offset(&self) -> Option<NonZeroU64> {
        self.0.extension_offset
    }

    pub fn set_extension_offset(&mut self, value: Option<NonZeroU64>) {
        self.0.extension_offset = value;
    }

    pub fn count(&self) -> u32 {
        self.0.count
    }

    pub fn set_count(&mut self, value: u32) {
        self.0.count = value;
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(bytes_of(&self.0))?;
        Ok(())
    }
}

#[repr(transparent)]
pub struct InterfaceEntry(InterfaceEntryRaw);

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct InterfaceEntryRaw {
    type_uuid: [u8; 16],
    version_major: u16,
    version_minor: u16,
    data: [u8; 8],
}

impl InterfaceEntry {
    pub fn new(type_uuid: Uuid, version: Version) -> Self {
        let mut value = InterfaceEntry(Zeroable::zeroed());
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

    pub fn data(&mut self) -> [u8; 8] {
        self.0.data
    }

    pub fn set_data(&mut self, data: [u8; 8]) {
        self.0.data = data;
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(bytes_of(&self.0))?;
        Ok(())
    }
}
