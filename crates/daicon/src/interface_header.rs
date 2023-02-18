use std::num::NonZeroU64;

use bytemuck::{bytes_of, from_bytes, from_bytes_mut, Pod, TransparentWrapper, Zeroable};

#[repr(transparent)]
pub struct InterfaceTableHeader(InterfaceTableHeaderRaw);

impl InterfaceTableHeader {
    pub fn new() -> Self {
        let value = Self(Zeroable::zeroed());
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

    pub fn extension_offset(&self) -> Option<NonZeroU64> {
        self.0.extension_offset
    }

    pub fn set_extension_offset(&mut self, value: Option<NonZeroU64>) {
        self.0.extension_offset = value;
    }

    pub fn extension_count_hint(&self) -> u32 {
        self.0.extension_count_hint
    }

    pub fn set_extension_count_hint(&mut self, value: u32) {
        self.0.extension_count_hint = value;
    }

    pub fn count(&self) -> u32 {
        self.0.count
    }

    pub fn set_count(&mut self, value: u32) {
        self.0.count = value;
    }

    pub fn region_offset(&self) -> u64 {
        self.0.region_offset
    }

    pub fn set_region_offset(&mut self, value: u64) {
        self.0.region_offset = value;
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct InterfaceTableHeaderRaw {
    extension_offset: Option<NonZeroU64>,
    extension_count_hint: u32,
    count: u32,
    region_offset: u64,
}

unsafe impl TransparentWrapper<InterfaceTableHeaderRaw> for InterfaceTableHeader {}
