//! Dacti index component low-level types, for zero-copy reading and writing.
//!
//! This library does not guarantee 100% correctness in input or output, but will provide minimal
//! validation where useful. In most cases, you should not use this library directly, but instead
//! use a format-specific library that uses this library.
//!
//! Where possible, high-level wrappers are `#[repr(transparent)]` to the low-level data of those
//! types, and can be reinterpreted. However, you should avoid doing this. Safe raw binary
//! conversion can be done instead with `from_bytes`, `from_bytes_mut`, and `Deref`.
//!
//! This library version is based off the dacti package format 0.1.0-draft specification.

mod entry;
mod group;
mod header;

use uuid::{uuid, Uuid};

pub use self::{
    entry::IndexEntry,
    group::{IndexGroupEncoding, IndexGroupHeader},
    header::IndexHeader,
};

pub const INDEX_COMPONENT_UUID: Uuid = uuid!("2c5e4717-b715-429b-85cd-d320d242547a");
