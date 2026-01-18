use crate::{
    lang::{VerbPhrase, TransitiveVerbPhrase, TransitiveVerb, verbs::ToCollect},
    world::{handle::WorldObjectHandle, World, WorldObjectGetError},
    worldobject::{
        fns::update::Action,
        components::controllers::commands::collect_command::CollectCommand
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
                        let location = world.locate_object(&target_handle)?;
                        let object = world.take_object(&target_handle)?;
                    
                        let inventory_item = object.collect().await
                            .or_else(|(err, og_object)| {
                                world.add_object(target_handle, og_object, location);
                                Err(err)
                        })?;
                    
                        world.give_item_to(&my_handle, inventory_item)
                            .map_err(|err| Box::new(err))?;
                    
                        Ok(None)
                    })
                }
            )
        },
        verb_phrase: VerbPhrase::Transitive(
            TransitiveVerbPhrase {
                verb: TransitiveVerb::new(ToCollect),
                direct_object: world.get_object(&cmd.target_handle)
                    .map(|object| object.definite_description())
                    .map_err(|err| CollectCommandToActionError::FailedToGetTargetObject(err))?
            }
        )
    })
}
