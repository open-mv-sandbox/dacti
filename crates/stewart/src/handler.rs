use std::any::Any;

use crate::Context;

pub trait Handler: Any + Send + Sync {
    type Message: Any;

    fn handle(&mut self, context: &Context, message: Self::Message);
}
