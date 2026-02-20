use std::fmt;

use serde::Serialize;
use serde::Deserialize;

use crate::world::handle::WorldObjectHandle;

#[derive(Serialize, Deserialize)]
pub struct ExamineCommand {
    pub target_handle: WorldObjectHandle
}

#[derive(Debug)]
pub enum ExamineCommandParseError {
    NoObjectHandleProvided,
    InvalidObjectHandle(String),
}

impl fmt::Display for ExamineCommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoObjectHandleProvided => write!(f, "no object handle provided"),
            Self::InvalidObjectHandle(handle_str) => write!(f, "invalid object handle \"{}\"", handle_str)
        }
    }
}

impl ExamineCommand {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, ExamineCommandParseError> {
        let target_handle = words.next().ok_or(ExamineCommandParseError::NoObjectHandleProvided)?;
        let target_handle = WorldObjectHandle::try_from(target_handle)
            .map_err(|_| ExamineCommandParseError::InvalidObjectHandle(target_handle.to_string()))?;
        Ok(ExamineCommand { target_handle })
    }
}