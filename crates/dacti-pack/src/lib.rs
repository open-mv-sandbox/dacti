mod entry;
mod group;
mod header;

use uuid::{uuid, Uuid};

pub use self::{
    entry::IndexEntry,
    group::{IndexGroup, IndexGroupEncoding},
    header::IndexComponentHeader,
};

pub const INDEX_COMPONENT_UUID: Uuid = uuid!("2c5e4717-b715-429b-85cd-d320d242547a");
