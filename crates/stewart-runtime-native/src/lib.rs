//! Native runtime for stewart.

mod actors;
mod manager;
mod runtime;

pub use self::runtime::{NativeDispatcher, NativeRuntime};
