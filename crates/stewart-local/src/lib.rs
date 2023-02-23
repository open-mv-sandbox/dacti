//! Stewart APIs for starting and communicating with actors locally.
//!
//! These messages are not required to be used or supported, but for interoperability you should
//! prefer these over custom messages.

mod address;
mod start;

use std::{any::Any, sync::Arc};

pub use self::{
    address::Address,
    start::{AnyActor, StartActor},
};

/// Generic interface for sending messages to addresses.
pub trait Dispatcher {
    fn send_any(&self, address: usize, message: Box<dyn Any>);
}

impl dyn Dispatcher {
    pub fn send<M: 'static>(&self, address: Address<M>, message: M) {
        self.send_any(address.to_raw(), Box::new(message));
    }
}

/// Helper alias for a shared dynamic dispatcher.
///
/// TODO: This is kinda wonky, but it's almost universal for using a dispatcher.
/// Is there a better way?
pub type DispatcherArc = Arc<dyn Dispatcher>;
