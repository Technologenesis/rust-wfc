use std::fmt;

use serde::Serialize;
use serde::Deserialize;

use crate::{
    lang::{VerbPhrase, TransitiveVerbPhrase, TransitiveVerb, verbs::ToCollect},
    world::{handle::WorldObjectHandle, World},
    worldobject::{fns::update::Action, WorldObject, human::UnsouledHuman}
};

#[derive(Serialize, Deserialize)]
pub struct CollectAction {
    pub target_handle: WorldObjectHandle
}

#[derive(Debug)]
pub enum CollectActionParseError {
    NoObjectHandleProvided,
    InvalidObjectHandle(String),
}

impl fmt::Display for CollectActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoObjectHandleProvided => write!(f, "no object handle provided"),
            Self::InvalidObjectHandle(handle_str) => write!(f, "invalid object handle \"{}\"", handle_str)
        }
    }
}

impl CollectAction {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, CollectActionParseError> {
        let target_handle = words.next().ok_or(CollectActionParseError::NoObjectHandleProvided)?;
        let target_handle = WorldObjectHandle::try_from(target_handle)
            .map_err(|_| CollectActionParseError::InvalidObjectHandle(target_handle.to_string()))?;
        Ok(CollectAction { target_handle })
    }

    pub fn to_action(self, my_handle: WorldObjectHandle, target: &dyn WorldObject) -> Action {
        Action{
            exec: Box::new(
                move |world: &mut World| {
                    Box::pin(async move {
                        let location = world.locate_object(&self.target_handle)?;
                        let object = world.take_object(&self.target_handle)?;

                        let inventory_item = object.collect().await
                            .or_else(|(err, og_object)| {
                                world.add_object(self.target_handle.clone(), og_object, location);
                                Err(err)
                        })?;

                        world.give_item_to(&my_handle, inventory_item)
                            .map_err(|err| Box::new(err))?;

                        Ok(None)
                    })
                }
            ),
            verb_phrase: VerbPhrase::Transitive(
                TransitiveVerbPhrase {
                    verb: TransitiveVerb::new(ToCollect),
                    direct_object: target.definite_description()
                }
            )
        }
    }
}

