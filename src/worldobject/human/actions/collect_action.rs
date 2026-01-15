use crate::{
    world::{
        handle::WorldObjectHandle,
        World
    }
};

use std::fmt;

use serde::Serialize;
use serde::Deserialize;

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
}