//! Stewart APIs for starting and communicating with actors locally.
//!
//! These messages are not required to be used or supported, but for interoperability you should
//! prefer these over custom messages.

mod start;

pub use self::start::{AnyActor, StartActor};
