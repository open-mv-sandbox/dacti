//! Daicon low-level types, for zero-copy reading and writing.
//!
//! This library does not guarantee 100% correctness in input or output, but will provide minimal
//! validation where useful. In most cases, you should not use this library directly, but instead
//! use a format-specific library that uses this library.
//!
//! Endianness compatibility is maintained when setting and reading values, while maintaing
//! near-native performance on little-endian systems in release build mode.
//!
//! Where possible, high-level wrappers are `#[repr(transparent)]` to the low-level data of those
//! types, and can be reinterpreted. However, you should avoid doing this. Safe raw binary
//! conversion can be done instead with `from_bytes`, `from_bytes_mut`, `as_bytes`, and
//! `as_bytes_mut`.

mod entry;
mod header;

use bytemuck::{Pod, TransparentWrapper, Zeroable};
use wrapmuck::Wrapmuck;

pub use self::{entry::ComponentEntry, header::ComponentTableHeader};

/// Signature of a daicon file, should be inserted and validated at the top of a file.
pub const SIGNATURE: &[u8] = b"\xFFdaicon0";

/// Common 'region' data contents.
#[derive(TransparentWrapper, Wrapmuck, Debug, Clone)]
#[repr(transparent)]
pub struct RegionData(RegionDataRaw);

impl RegionData {
    pub fn offset(&self) -> u32 {
        self.0.offset
    }

    pub fn set_offset(&mut self, value: u32) {
        self.0.offset = value.to_le();
    }

    pub fn size(&self) -> u32 {
        self.0.size
    }

    pub fn set_size(&mut self, value: u32) {
        self.0.size = value.to_le();
    }
}

#[derive(Pod, Zeroable, Debug, Clone, Copy)]
#[repr(C)]
struct RegionDataRaw {
    offset: u32,
    size: u32,
}
