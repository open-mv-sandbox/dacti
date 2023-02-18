mod entry;
mod header;
mod region;

use uuid::{uuid, Uuid};

pub use self::{
    entry::IndexComponentEntry, header::IndexComponentHeader, region::IndexComponentRegion,
};

pub const INDEX_COMPONENT_ID: Uuid = uuid!("2c5e4717-b715-429b-85cd-d320d242547a");
