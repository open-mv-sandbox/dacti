use bytemuck::{Pod, Zeroable};

#[derive(Pod, Zeroable, Debug, Clone, Copy)]
#[repr(C)]
pub struct IndexHeader {
    version: u32,
    groups: u32,
}

impl IndexHeader {
    pub fn version(&self) -> u32 {
        u32::from_le(self.version)
    }

    pub fn set_version(&mut self, value: u32) {
        self.version = value.to_le();
    }

    pub fn groups(&self) -> u32 {
        u32::from_le(self.groups)
    }

    pub fn set_groups(&mut self, value: u32) {
        self.groups = value.to_le();
    }
}
