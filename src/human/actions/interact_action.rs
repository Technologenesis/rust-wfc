use crate::world;

use serde::Serialize;
use serde::Deserialize;

use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct InteractAction {
    pub target_handle: world::WorldObjectHandle
}

#[derive(Debug)]
pub enum InteractActionParseError {
    NoObjectHandleProvided,
    InvalidObjectHandle(String),
}

impl fmt::Display for InteractActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InteractActionParseError::InvalidObjectHandle(handle) => write!(f, "invalid object handle: {}", handle),
            InteractActionParseError::NoObjectHandleProvided => write!(f, "no object handle provided")
        }
    }
}

impl InteractAction {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, InteractActionParseError> {
        let target_handle = words.next().ok_or(InteractActionParseError::NoObjectHandleProvided)?;
        let target_handle = world::WorldObjectHandle::try_from(target_handle)
            .map_err(|_| InteractActionParseError::InvalidObjectHandle(target_handle.to_string()))?;
        Ok(InteractAction { target_handle })
    }
}