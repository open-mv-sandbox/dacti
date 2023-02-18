//! Daicon writing and reading utility library.
//!
//! This library does not guarantee 100% correctness in input or output, but will provide minimal
//! validation where useful. In most cases, you should not use this library directly, but instead
//! use a format-specific library that uses this library.
//!
//! Endianness compatibility is maintained when setting and reading values, while maintaing near-
//! native performance on little-endian systems in release build mode.
//!
//! Where possible, high-level wrappers are `#[repr(transparent)]` to the low-level data of those
//! types, and can be reinterpreted. However, you should avoid doing this. Safe raw binary
//! conversion can be done instead with `from_bytes`, `from_bytes_mut` and `to_bytes`.

mod interface_entry;
mod interface_header;

pub use self::{interface_entry::InterfaceEntry, interface_header::InterfaceTableHeader};

/// Signature of a daicon file, should be inserted and validated at the top of a file.
pub const SIGNATURE: &[u8] = b"\xFFdaicon0";

/// Semantic version for formats and interfaces.
///
/// Only contains major and minor, see daicon spec for reasoning.
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
