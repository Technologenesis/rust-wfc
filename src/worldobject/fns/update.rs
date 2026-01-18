use futures::future::BoxFuture;

use crate::{
    lang::VerbPhrase,
    lang::TransitiveVerbPhrase,
    lang::TransitiveVerb,
    lang::verbs::ToDo,
    world::{World}
};

use super::Error;

pub struct Action {
    pub exec: Box<dyn FnOnce(&mut World) -> BoxFuture<
        Result<Option<String>, Error>
    >>,
    pub verb_phrase: VerbPhrase
}

impl Action {
    pub async fn call(self, world: &mut World) -> Result<Option<String>, Error> {
        (self.exec)(world).await
    }

    pub fn no_op() -> Self {
        Self {
            exec: Box::new(|_| Box::pin(async { Ok(None) })),
            verb_phrase: VerbPhrase::Transitive(
                TransitiveVerbPhrase {
                    verb: TransitiveVerb::new(ToDo),
                    direct_object: String::from("nothing"),
                }
            )
        }
    }
}