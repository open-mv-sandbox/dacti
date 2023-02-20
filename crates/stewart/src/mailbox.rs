use std::any::Any;

use anyhow::Error;

use crate::Context;

pub trait Actor: Send + Sync + 'static {
    type Message: Any;

    fn handle(&self, ctx: &Context, message: Self::Message) -> Result<(), Error>;
}
