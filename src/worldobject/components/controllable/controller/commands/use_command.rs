use std::fmt;

use serde::Serialize;
use serde::Deserialize;

use crate::world::handle::WorldObjectHandle;



#[derive(Serialize, Deserialize)]
pub struct UseCommand {
    pub item_name: String,
    pub target_handle: Option<WorldObjectHandle>
}

#[derive(Debug)]
pub enum UseCommandParseError {
    NoItemNameProvided,
    InvalidItemName(String),
    InvalidTargetHandle(Box<dyn std::error::Error>),
}

impl fmt::Display for UseCommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UseCommandParseError::InvalidItemName(name) => write!(f, "you are not wielding an item named \"{}\"", name),
            UseCommandParseError::NoItemNameProvided => write!(f, "no item name provided"),
            UseCommandParseError::InvalidTargetHandle(err) => write!(f, "invalid target handle: {}", err)
        }
    }
}   

impl UseCommand {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, UseCommandParseError> {
        let item_name = words.next().ok_or(UseCommandParseError::NoItemNameProvided)?;
        let target_handle = words.next()
            .map(|handle| WorldObjectHandle::try_from(handle))
            .transpose()
            .map_err(|err| UseCommandParseError::InvalidTargetHandle(Box::new(err)))?;
        Ok(UseCommand {
            item_name: String::from(item_name),
            target_handle
        })
    }
}