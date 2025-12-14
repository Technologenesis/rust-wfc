use std::fmt;

use serde::Serialize;
use serde::Deserialize;

use crate::world;
use crate::quantities::direction;

#[derive(Serialize, Deserialize)]
pub struct AttackAction {
    pub target_handle: world::WorldObjectHandle,
    pub left_or_right_arm: Option<direction::DirectionHorizontal>
}

#[derive(Debug)]
pub enum AttackActionParseError {
    NoObjectHandleProvided,
    InvalidObjectHandle(String),
    InvalidHandedness(direction::InvalidHorizontalDirectionError),
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
        let target_handle = world::WorldObjectHandle::try_from(target_handle)
            .map_err(|_| AttackActionParseError::InvalidObjectHandle(target_handle.to_string()))?;

        let left_or_right_arm = words.next().map(
            |left_or_right_arm| direction::DirectionHorizontal::try_from(left_or_right_arm)
                .map_err(|err| AttackActionParseError::InvalidHandedness(err))
        ).transpose()?;

        Ok(AttackAction { target_handle, left_or_right_arm })
    }
}