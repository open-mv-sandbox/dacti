use bytemuck::{Pod, Zeroable};
use uuid::Uuid;

#[derive(Pod, Zeroable, Debug, Clone, Copy)]
#[repr(C)]
pub struct IndexEntry {
    region_id: [u8; 16],
    offset: u32,
    size: u32,
}

impl IndexEntry {
    pub fn region_id(&self) -> Uuid {
        Uuid::from_bytes_le(self.region_id)
    }

    pub fn set_region_id(&mut self, value: Uuid) {
        self.region_id = value.to_bytes_le();
    }

    pub fn offset(&self) -> u32 {
        u32::from_le(self.offset)
    }

    pub fn set_offset(&mut self, value: u32) {
        self.offset = value.to_le();
    }

    pub fn size(&self) -> u32 {
        u32::from_le(self.size)
    }

    pub fn set_size(&mut self, value: u32) {
        self.size = value.to_le();
    }
}
