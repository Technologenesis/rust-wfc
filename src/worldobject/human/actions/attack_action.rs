use std::fmt;

use serde::Serialize;
use serde::Deserialize;

use crate::{
    lang::{VerbPhrase, TransitiveVerbPhrase, TransitiveVerb, verbs::ToAttack},
    worldobject::{fns::update::Action, WorldObject, human::unsouled::UnsouledHuman},
    world::{World, handle::WorldObjectHandle},
    quantities::direction::{DirectionHorizontal, InvalidHorizontalDirectionError}
};

#[derive(Serialize, Deserialize)]
pub struct AttackAction {
    pub target_handle: WorldObjectHandle,
    pub left_or_right_arm: Option<DirectionHorizontal>
}

#[derive(Debug)]
pub enum AttackError {
    NoArmProvided,
}

impl std::fmt::Display for AttackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoArmProvided => write!(f, "no arm provided"),
        }
    }
}

impl std::error::Error for AttackError {}

#[derive(Debug)]
pub enum AttackActionParseError {
    NoObjectHandleProvided,
    InvalidObjectHandle(String),
    InvalidHandedness(InvalidHorizontalDirectionError),
}

impl fmt::Display for AttackActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoObjectHandleProvided => write!(f, "no object handle provided"),
            Self::InvalidObjectHandle(handle_str) => write!(f, "invalid object handle \"{}\"", handle_str),
            Self::InvalidHandedness(err) => write!(f, "invalid handedness: {}", err)
        }
    }
}

impl AttackAction {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, AttackActionParseError> {
        let target_handle = words.next().ok_or(AttackActionParseError::NoObjectHandleProvided)?;
        let target_handle = WorldObjectHandle::try_from(target_handle)
            .map_err(|_| AttackActionParseError::InvalidObjectHandle(target_handle.to_string()))?;

        let left_or_right_arm = words.next().map(
            |left_or_right_arm| DirectionHorizontal::try_from(left_or_right_arm)
                .map_err(|err| AttackActionParseError::InvalidHandedness(err))
        ).transpose()?;

        Ok(AttackAction { target_handle, left_or_right_arm })
    }

    pub fn to_action(self, me: UnsouledHuman, target: &dyn WorldObject) -> Action {
        Action{
            exec: Box::new(
                move |world: &mut World| {
                    Box::pin(async move {
                        let arm = match self.left_or_right_arm.as_ref().unwrap_or(&me.dominant_arm) {
                            DirectionHorizontal::Left => &me.arm_left,
                            DirectionHorizontal::Right => &me.arm_right,
                        }.as_ref().ok_or(AttackError::NoArmProvided)?;
                    
                        let punch_force = arm.punch_force.clone();
                        
                        let object = world.get_object_mut(&self.target_handle)
                            .map_err(|err| Box::new(err))?;
                    
                        let msg = object.apply_force(&punch_force).await
                            .unwrap_or_else(|err| format!("failed to apply force: {}", err));
                    
                        Ok(Some(msg))
                    })
                }
            ),
            verb_phrase: VerbPhrase::Transitive(
                TransitiveVerbPhrase {
                    verb: TransitiveVerb::new(ToAttack),
                    direct_object: target.definite_description()
                }
            )
        }
    }
}

