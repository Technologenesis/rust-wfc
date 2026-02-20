use crate::{
    lang::{VerbPhrase, TransitiveVerbPhrase, TransitiveVerb, verbs::ToCollect},
    world::{handle::WorldObjectHandle, World, WorldObjectGetError},
    worldobject::{
        fns::update::Action,
        components::controllable::controller::commands::collect_command::CollectCommand
    }
};

#[derive(Debug)]
pub enum CollectCommandToActionError {
    FailedToGetTargetObject(WorldObjectGetError),
}

impl std::fmt::Display for CollectCommandToActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToGetTargetObject(err) => write!(f, "failed to get target object: {}", err),
        }
    }
}

impl std::error::Error for CollectCommandToActionError {}

pub fn from_command(cmd: CollectCommand, world: &World, my_handle: WorldObjectHandle) -> Result<Action, CollectCommandToActionError> {
    Ok(Action{
        exec: {
            let target_handle = cmd.target_handle.clone();
            Box::new(
                move |world: &mut World| {
                    Box::pin(async move {
                        let _: () = todo!("collect via component trait");
                        Ok(None)
                    })
                }
            )
        },
        verb_phrase: VerbPhrase::Transitive(
            TransitiveVerbPhrase {
                verb: TransitiveVerb::new(ToCollect),
                direct_object: world.get_object(&cmd.target_handle)
                    .map(|object| object.linguistics().definite_description.clone())
                    .map_err(|err| CollectCommandToActionError::FailedToGetTargetObject(err))?
            }
        )
    })
}
