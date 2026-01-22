use crate::{
    worldobject::{fns::update::Action, human::Human,
        components::controllers::commands::use_command::UseCommand
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

pub fn from_command(me: &mut Human, my_handle: WorldObjectHandle, cmd: UseCommand, world: &World) -> Result<Action, UseCommandToActionError> {
    let wielded_item = me.wielded_items_mut().find(
        |item| item.name() == cmd.item_name
    ).ok_or(UseCommandToActionError::FailedToFindWieldedItem(cmd.item_name))?;
    
    wielded_item.use_item(world, my_handle, cmd.target_handle).map_err(|err| UseCommandToActionError::FailedToUseItem(err))
}

