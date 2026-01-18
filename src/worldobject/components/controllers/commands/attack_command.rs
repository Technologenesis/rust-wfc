use std::fmt;

use serde::Serialize;
use serde::Deserialize;

use crate::world::handle::WorldObjectHandle;

#[derive(Serialize, Deserialize)]
pub struct AttackCommand {
    pub target_handle: WorldObjectHandle,
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
    InvalidObjectHandle(String)
}

impl fmt::Display for AttackActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoObjectHandleProvided => write!(f, "no object handle provided"),
            Self::InvalidObjectHandle(handle_str) => write!(f, "invalid object handle \"{}\"", handle_str)
        }
    }
}

impl AttackCommand {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, AttackActionParseError> {
        let target_handle = words.next().ok_or(AttackActionParseError::NoObjectHandleProvided)?;
        let target_handle = WorldObjectHandle::try_from(target_handle)
            .map_err(|_| AttackActionParseError::InvalidObjectHandle(target_handle.to_string()))?;

        Ok(AttackCommand { target_handle })
    }
}

