use bytemuck::{Pod, TransparentWrapper, Zeroable};
use wrapmuck::Wrapmuck;

#[derive(TransparentWrapper, Wrapmuck, Debug, Clone)]
#[repr(transparent)]
pub struct IndexComponentHeader(IndexComponentHeaderRaw);

impl IndexComponentHeader {
    pub fn version(&self) -> u32 {
        u32::from_le(self.0.version)
    }

    pub fn set_version(&mut self, value: u32) {
        self.0.version = value.to_le();
    }

    pub fn groups(&self) -> u32 {
        u32::from_le(self.0.groups)
    }

    pub fn set_groups(&mut self, value: u32) {
        self.0.groups = value.to_le();
    }
}

#[derive(Pod, Zeroable, Debug, Clone, Copy)]
#[repr(C)]
struct IndexComponentHeaderRaw {
    version: u32,
    // TODO: Group spacing, currently adding a file may require group reorder, violating append-only
    groups: u32,
}
