use futures::future::BoxFuture;

use crate::{
    world::{World, WorldObjectGetError},
    worldobject::{
        Error as WorldObjectError,
        components::controllers::commands::interact_action::InteractAction,
        fns::update::Action
    },
    lang::{VerbPhrase, PrepositionalVerbPhrase, PrepositionalPhrase, IntransitiveVerb, verbs::ToInteract}
};

#[derive(Debug)]
pub enum InteractCommandToActionError {
    FailedToGetTargetObject(WorldObjectGetError),
}

impl std::fmt::Display for InteractCommandToActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToGetTargetObject(err) => write!(f, "failed to get target object: {}", err),
        }
    }
}

impl std::error::Error for InteractCommandToActionError {}

pub fn from_command(cmd: InteractAction, world: &World) -> Result<Action, InteractCommandToActionError> {
    Ok(Action{
        exec: {
            let target_handle = cmd.target_handle.clone();
            Box::new(
                move |world: &mut World| -> BoxFuture<Result<Option<String>, WorldObjectError>> {
                    Box::pin(async move {
                        let object = world.get_object_mut(&target_handle)
                            .map_err(|err| Box::new(err))?;
                    
                        let msg = object.interact().await?;

                        Ok(Some(msg))
                    })
                }
            )
        },
        verb_phrase: VerbPhrase::Prepositional(
            PrepositionalVerbPhrase {
                main_verb_phrase: Box::new(
                    VerbPhrase::Intransitive(
                        IntransitiveVerb::new(ToInteract)
                    )
                ),
                prepositional_phrase: PrepositionalPhrase {
                    preposition: String::from("with"),
                    object: world.get_object(&cmd.target_handle)
                        .map(|object| object.definite_description())
                        .map_err(|err| InteractCommandToActionError::FailedToGetTargetObject(err))?
                }
            }
        )
    })
}