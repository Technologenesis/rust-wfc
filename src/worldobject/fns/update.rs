use futures::future::BoxFuture;

use crate::{
    lang::VerbPhrase,
    world::World
};

use super::Error;

pub struct Action {
    exec: Box<dyn FnOnce(&mut World) -> BoxFuture<
        Result<Option<String>, Error>
    >>,
    pub verb_phrase: VerbPhrase
}

pub struct UpdateFn(pub Box<dyn FnOnce(&mut World) -> BoxFuture<
    Result<Option<String>, Error>
>>);

impl UpdateFn {
    pub async fn call(self, world: &mut World) -> Result<Option<String>, Error> {
        (self.0)(world).await
    }

    pub fn no_op() -> Self {
        Self(Box::new(|_| Box::pin(async { Ok(None) })))
    }
}