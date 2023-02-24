use bytemuck::{Pod, TransparentWrapper, Zeroable};
use uuid::Uuid;
use wrapmuck::Wrapmuck;

#[derive(TransparentWrapper, Wrapmuck, Debug, Clone)]
#[repr(transparent)]
pub struct IndexEntry(IndexEntryRaw);

impl IndexEntry {
    pub fn region_id(&self) -> Uuid {
        Uuid::from_bytes_le(self.0.region_id)
    }

    pub fn set_region_id(&mut self, value: Uuid) {
        self.0.region_id = value.to_bytes_le();
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
struct IndexEntryRaw {
    region_id: [u8; 16],
    offset: u32,
    size: u32,
}
