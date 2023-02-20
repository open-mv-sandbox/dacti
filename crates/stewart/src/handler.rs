use std::any::Any;

use anyhow::Error;

use crate::Context;

pub trait Handler: Send + Sync {
    type Message: Any;

    fn handle(&mut self, ctx: &Context, message: Self::Message) -> Result<(), Error>;
}
