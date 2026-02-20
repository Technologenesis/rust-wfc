use crate::{
    worldobject::{
        components::{
            controllable::controller::commands::wield_command::WieldCommand,
            container::ContainerHandle,
        },
        fns::update::Action,
        human::Human,
    },
};

#[derive(Debug)]
pub enum WieldCommandToActionError {
    NoSuchItem(ContainerHandle),
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

pub fn from_command(_me: &mut Human, _cmd: WieldCommand) -> Result<Action, WieldCommandToActionError> {
    todo!("wield item via ContainerHandle")
}
