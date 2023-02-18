use bytemuck::{Pod, TransparentWrapper, Zeroable};
use uuid::Uuid;
use wrapmuck::Wrapmuck;

/// Entry in the component table.
#[derive(TransparentWrapper, Wrapmuck, Debug, Clone)]
#[repr(transparent)]
pub struct ComponentEntry(ComponentEntryRaw);

impl ComponentEntry {
    pub fn type_uuid(&self) -> Uuid {
        Uuid::from_bytes_le(self.0.type_uuid)
    }

    pub fn set_type_uuid(&mut self, value: Uuid) {
        self.0.type_uuid = value.to_bytes_le();
    }

    pub fn data(&self) -> &[u8; 8] {
        &self.0.data
    }

    pub fn data_mut(&mut self) -> &mut [u8; 8] {
        &mut self.0.data
    }
}

#[derive(Pod, Zeroable, Debug, Clone, Copy)]
#[repr(C)]
struct ComponentEntryRaw {
    type_uuid: [u8; 16],
    data: [u8; 8],
}
