use std::fmt;

use serde::Serialize;
use serde::Deserialize;

use crate::worldobject::components::inventory::item::InventoryItemHandle;



#[derive(Serialize, Deserialize)]
pub struct WieldCommand {
    pub item_handle: InventoryItemHandle
}

#[derive(Debug)]
pub enum WieldCommandParseError {
    NoItemHandleProvided,
    InvalidItemHandle(String),
}

impl fmt::Display for WieldCommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WieldCommandParseError::InvalidItemHandle(handle) => write!(f, "invalid item handle: {}", handle),
            WieldCommandParseError::NoItemHandleProvided => write!(f, "no item handle provided")
        }
    }
}   

impl WieldCommand {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, WieldCommandParseError> {
        let item_handle = words.next().ok_or(WieldCommandParseError::NoItemHandleProvided)
            .and_then(|handle| InventoryItemHandle::try_from(handle)
                .map_err(|err| WieldCommandParseError::InvalidItemHandle(err))
            )?;
        Ok(WieldCommand { item_handle })
    }
}