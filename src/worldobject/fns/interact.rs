use std::future::Future;

use crate::world::World;

use super::Error;

pub struct InteractFn(fn(&mut World) -> Box<dyn Future<
    Output = Result<(), Error>
>>);