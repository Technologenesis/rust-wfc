use futures::future::BoxFuture;

use crate::{
    lang::{TransitiveVerb, TransitiveVerbPhrase, VerbPhrase, verbs::ToWield}, world::World, worldobject::{
        Error as WorldObjectError, components::{
            controllers::commands::wield_command::WieldCommand,
            inventory::item::InventoryItemHandle
        }, fns::update::Action, human::Human
    },
    quantities::direction::DirectionHorizontal,
};

#[derive(Debug)]
pub enum WieldCommandToActionError {
    NoSuchItem(InventoryItemHandle),
    FailedToWieldItem(Box<dyn std::error::Error>),
}

impl std::fmt::Display for WieldCommandToActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchItem(handle) => write!(f, "inventory does not contain item: {}", handle),
            Self::FailedToWieldItem(err) => write!(f, "failed to wield item: {}", err),
        }
    }
}

impl std::error::Error for WieldCommandToActionError {}

pub fn from_command(me: &mut Human, cmd: WieldCommand) -> Result<Action, WieldCommandToActionError> {
}