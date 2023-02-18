use std::num::NonZeroU64;

use bytemuck::{Pod, TransparentWrapper, Zeroable};
use wrapmuck::Wrapmuck;

/// Header of the component table.
#[derive(TransparentWrapper, Wrapmuck, Clone)]
#[repr(transparent)]
pub struct ComponentTableHeader(ComponentTableHeaderRaw);

impl ComponentTableHeader {
    pub fn extension_offset(&self) -> Option<NonZeroU64> {
        let value = u64::from_le(self.0.extension_offset);
        NonZeroU64::new(value)
    }

    pub fn set_extension_offset(&mut self, value: Option<NonZeroU64>) {
        self.0.extension_offset = value.map(|v| v.get()).unwrap_or(0);
    }

    pub fn extension_count_hint(&self) -> u32 {
        u32::from_le(self.0.extension_count_hint)
    }

    pub fn set_extension_count_hint(&mut self, value: u32) {
        self.0.extension_count_hint = value.to_le();
    }

    pub fn length(&self) -> u32 {
        u32::from_le(self.0.length)
    }

    pub fn set_length(&mut self, value: u32) {
        self.0.length = value.to_le();
    }

    pub fn region_offset(&self) -> u64 {
        u64::from_le(self.0.region_offset)
    }

    pub fn set_region_offset(&mut self, value: u64) {
        self.0.region_offset = value.to_le();
    }
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct ComponentTableHeaderRaw {
    extension_offset: u64,
    extension_count_hint: u32,
    length: u32,
    region_offset: u64,
}
