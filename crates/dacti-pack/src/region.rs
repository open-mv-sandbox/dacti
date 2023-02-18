use bytemuck::{Pod, TransparentWrapper, Zeroable};
use wrapmuck::Wrapmuck;

#[derive(TransparentWrapper, Wrapmuck, Debug, Clone)]
#[repr(transparent)]
pub struct IndexComponentRegion(IndexComponentRegionRaw);

impl IndexComponentRegion {
    pub fn offset(&self) -> u64 {
        u64::from_le(self.0.offset)
    }

    pub fn set_offset(&mut self, value: u64) {
        self.0.offset = value.to_le();
    }

    pub fn length(&self) -> u32 {
        u32::from_le(self.0.length)
    }

    pub fn set_length(&mut self, value: u32) {
        self.0.length = value.to_le();
    }
}

#[derive(Pod, Zeroable, Debug, Clone, Copy)]
#[repr(C)]
struct IndexComponentRegionRaw {
    offset: u64,
    reserved: u32,
    length: u32,
}
