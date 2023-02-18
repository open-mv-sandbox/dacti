use bytemuck::{bytes_of, from_bytes, from_bytes_mut, Pod, TransparentWrapper, Zeroable};
use uuid::Uuid;

#[repr(transparent)]
pub struct ComponentEntry(ComponentEntryRaw);

impl ComponentEntry {
    pub fn new(type_uuid: Uuid) -> Self {
        let mut value = ComponentEntry(Zeroable::zeroed());
        value.set_type_uuid(type_uuid);
        value
    }

    pub fn from_bytes(bytes: &[u8]) -> &Self {
        Self::wrap_ref(from_bytes(bytes))
    }

    pub fn from_bytes_mut(bytes: &mut [u8]) -> &Self {
        Self::wrap_mut(from_bytes_mut(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        bytes_of(&self.0)
    }

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

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct ComponentEntryRaw {
    type_uuid: [u8; 16],
    data: [u8; 8],
}

unsafe impl TransparentWrapper<ComponentEntryRaw> for ComponentEntry {}
