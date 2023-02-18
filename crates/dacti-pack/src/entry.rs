use bytemuck::{Pod, TransparentWrapper, Zeroable};
use uuid::Uuid;
use wrapmuck::Wrapmuck;

#[derive(TransparentWrapper, Wrapmuck, Debug, Clone)]
#[repr(transparent)]
pub struct IndexComponentEntry(IndexComponentEntryRaw);

impl IndexComponentEntry {
    pub fn uuid(&self) -> Uuid {
        Uuid::from_bytes_le(self.0.uuid)
    }

    pub fn set_uuid(&mut self, value: Uuid) {
        self.0.uuid = value.to_bytes_le();
    }

    pub fn offset(&self) -> u32 {
        u32::from_le(self.0.offset)
    }

    pub fn set_offset(&mut self, value: u32) {
        self.0.offset = value.to_le();
    }

    pub fn size(&self) -> u32 {
        u32::from_le(self.0.size)
    }

    pub fn set_size(&mut self, value: u32) {
        self.0.size = value.to_le();
    }
}

#[derive(Pod, Zeroable, Debug, Clone, Copy)]
#[repr(C)]
struct IndexComponentEntryRaw {
    uuid: [u8; 16],
    // TODO: This offset limits packages to 4GB, add 'regions' with relative offsets
    offset: u32,
    size: u32,
}
