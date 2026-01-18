use futures::future::BoxFuture;

use crate::{
    lang::{VerbPhrase, TransitiveVerbPhrase, TransitiveVerb, verbs::ToExamine},
    world::{World, WorldObjectGetError},
    worldobject::{
        Error as WorldObjectError,
        components::controllers::commands::examine_command::ExamineCommand,
        fns::update::Action
    }
};

#[derive(Debug)]
pub enum ExamineCommandToActionError {
    FailedToGetTargetObject(WorldObjectGetError),
}

impl std::fmt::Display for ExamineCommandToActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToGetTargetObject(err) => write!(f, "failed to get target object: {}", err),
        }
    }
}

impl std::error::Error for ExamineCommandToActionError {}

pub fn from_command(cmd: ExamineCommand, world: &World) -> Result<Action, ExamineCommandToActionError> {
    let target_description = world.get_object(&cmd.target_handle)
        .map(|object| object.definite_description())
        .map_err(|err| ExamineCommandToActionError::FailedToGetTargetObject(err))?;

    Ok(Action{
        exec: Box::new(
            move |world: &mut World| -> BoxFuture<Result<Option<String>, WorldObjectError>> {
                Box::pin(async move {
                    let object = world.get_object(&cmd.target_handle)?;
                    let msg = format!("you see {}", object.examine());
                    Ok(Some(msg))
                })
            }
        ),
        verb_phrase: VerbPhrase::Transitive(
            TransitiveVerbPhrase {
                verb: TransitiveVerb::new(ToExamine),
                direct_object: target_description
            }
        )
    })
}