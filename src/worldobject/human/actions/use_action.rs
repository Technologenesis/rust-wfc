use crate::{
    worldobject::{fns::update::Action, human::Human,
        components::controllable::controller::commands::use_command::UseCommand
    },
    world::{World, handle::WorldObjectHandle}
};

#[derive(Debug)]
pub enum UseCommandToActionError {
    FailedToFindWieldedItem(String),
    FailedToUseItem(Box<dyn std::error::Error>),
}

impl std::fmt::Display for UseCommandToActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToFindWieldedItem(name) => write!(f, "you are not wielding an item named \"{}\"", name),
            Self::FailedToUseItem(err) => write!(f, "failed to use item: {}", err),
        }
    }
}

impl std::error::Error for UseCommandToActionError {}

pub fn from_command(_me: &mut Human, _my_handle: WorldObjectHandle, _cmd: UseCommand, _world: &World) -> Result<Action, UseCommandToActionError> {
    todo!("use wielded item via new component API")
}

