use bytemuck::{bytes_of, from_bytes, from_bytes_mut, Pod, TransparentWrapper, Zeroable};
use uuid::Uuid;

use crate::Version;

#[repr(transparent)]
pub struct FormatHeader(FormatHeaderRaw);

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct FormatHeaderRaw {
    type_uuid: [u8; 16],
    version_major: u16,
    version_minor: u16,
}

unsafe impl TransparentWrapper<FormatHeaderRaw> for FormatHeader {}

impl FormatHeader {
    pub fn new(type_uuid: Uuid, version: Version) -> Self {
        let mut value = Self(Zeroable::zeroed());
        value.set_type_uuid(type_uuid);
        value.set_version(version);
        value
    }

    pub fn from_bytes(bytes: &[u8]) -> &Self {
        Self::wrap_ref(from_bytes(bytes))
    }

    pub fn from_bytes_mut(bytes: &mut [u8]) -> &Self {
        Self::wrap_mut(from_bytes_mut(bytes))
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

    pub fn as_bytes(&self) -> &[u8] {
        bytes_of(&self.0)
    }
}
