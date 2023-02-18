use bytemuck::{Pod, TransparentWrapper, Zeroable};
use uuid::Uuid;
use wrapmuck::Wrapmuck;

/// Entry in the component table.
#[derive(TransparentWrapper, Wrapmuck, Clone)]
#[repr(transparent)]
pub struct ComponentEntry(ComponentEntryRaw);

impl ComponentEntry {
    pub fn type_uuid(&self) -> Uuid {
        Uuid::from_bytes_le(self.0.type_uuid)
    }

    pub fn set_type_uuid(&mut self, value: Uuid) {
        self.0.type_uuid = value.to_bytes_le();
    }

    pub fn data(&mut self) -> [u8; 8] {
        self.0.data
    }

    pub fn set_data(&mut self, data: [u8; 8]) {
        self.0.data = data;
    }
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct ComponentEntryRaw {
    type_uuid: [u8; 16],
    data: [u8; 8],
}
