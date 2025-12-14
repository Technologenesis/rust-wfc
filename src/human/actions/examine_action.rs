use crate::world;

use std::fmt;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct ExamineAction {
    pub target_handle: world::WorldObjectHandle
}

#[derive(Debug)]
pub enum ExamineActionParseError {
    NoObjectHandleProvided,
    InvalidObjectHandle(String),
}

impl fmt::Display for ExamineActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoObjectHandleProvided => write!(f, "no object handle provided"),
            Self::InvalidObjectHandle(handle_str) => write!(f, "invalid object handle \"{}\"", handle_str)
        }
    }
}

impl ExamineAction {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, ExamineActionParseError> {
        let target_handle = words.next().ok_or(ExamineActionParseError::NoObjectHandleProvided)?;
        let target_handle = world::WorldObjectHandle::try_from(target_handle)
            .map_err(|_| ExamineActionParseError::InvalidObjectHandle(target_handle.to_string()))?;
        Ok(ExamineAction { target_handle })
    }
}