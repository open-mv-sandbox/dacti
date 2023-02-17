//! Daicon writing and reading utility library.
//!
//! This library does not guarantee 100% correctness in input or output, but will provide minimal
//! validation where useful. In most cases, you should not use this library directly, but instead
//! use a format-specific library that uses this library.
//!
//! This library does maintain endianness compatibility, while maintaing near-native performance
//! on little-endian systems in release build mode.
//!
//! Where possible, high-level types are `#[repr(transparent)]` to the low-level data of those
//! types, and can be safely reinterpreted. Internally, this library uses bytemuck, but this is
//! not exposed publically.

mod format;
mod interface_table;

pub use self::{
    format::FormatHeader,
    interface_table::{InterfaceEntry, InterfaceTableHeader},
};

pub const SIGNATURE: &[u8] = b"\xFFdaicon0";

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}

impl Version {
    pub fn new(major: u16, minor: u16) -> Self {
        Version { major, minor }
    }
}
