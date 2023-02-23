use std::{marker::PhantomData, sync::atomic::AtomicPtr};

/// An address a sender can use to route messages.
pub struct Address<M> {
    raw: usize,
    _p: PhantomData<AtomicPtr<M>>,
}

impl<M> Address<M> {
    pub fn from_raw(raw: usize) -> Self {
        Address {
            raw,
            _p: PhantomData,
        }
    }

    pub(crate) fn to_raw(&self) -> usize {
        self.raw
    }
}

impl<M> Clone for Address<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for Address<M> {}
