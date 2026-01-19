pub mod move_action;
pub mod interact_action;
pub mod collect_action;
pub mod attack_action;
pub mod examine_action;
pub mod circumspect_action;
pub mod inventory_action;
pub mod wield_action;
pub mod use_action;

use crate::{
    world::{World, handle::WorldObjectHandle}, worldobject::{
        TypedWorldObject, components::controllers::commands::Command, fns::update::Action, human::unsouled::UnsouledHuman
    }
};

use attack_action::AttackCommandToActionError;
use interact_action::InteractCommandToActionError;
use collect_action::CollectCommandToActionError;
use examine_action::ExamineCommandToActionError;
use wield_action::WieldCommandToActionError;
use use_action::UseCommandToActionError;

#[derive(Debug)]
pub enum CommandToActionError {
    AttackCommandToActionError(AttackCommandToActionError),
    InteractCommandToActionError(InteractCommandToActionError),
    CollectCommandToActionError(CollectCommandToActionError),
    ExamineCommandToActionError(ExamineCommandToActionError),
    WieldCommandToActionError(WieldCommandToActionError),
    UseCommandToActionError(UseCommandToActionError),
}

impl std::fmt::Display for CommandToActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AttackCommandToActionError(err) => write!(f, "failed to convert attack command to action: {}", err),
            Self::InteractCommandToActionError(err) => write!(f, "failed to convert interact command to action: {}", err),
            Self::CollectCommandToActionError(err) => write!(f, "failed to convert collect command to action: {}", err),
            Self::ExamineCommandToActionError(err) => write!(f, "failed to convert examine command to action: {}", err),
            Self::WieldCommandToActionError(err) => write!(f, "failed to convert wield command to action: {}", err),
            Self::UseCommandToActionError(err) => write!(f, "failed to convert use command to action: {}", err),
        }
    }
}

impl std::error::Error for CommandToActionError {}

impl super::Human {
    pub fn from_command(&mut self, cmd: Command, world: &World, my_handle: WorldObjectHandle) -> Result<Action, CommandToActionError> {
        match cmd {
            Command::Move(move_cmd) => Ok(move_action::from_command(move_cmd, my_handle, self.dummy())),
            Command::Interact(interact_cmd) => interact_action::from_command(interact_cmd, world)
                .map_err(CommandToActionError::InteractCommandToActionError),
            Command::Collect(collect_cmd) => collect_action::from_command(collect_cmd, world, my_handle)
                .map_err(CommandToActionError::CollectCommandToActionError),
            Command::Attack(attack_cmd) => attack_action::from_command(attack_cmd, world, self.dummy())
                .map_err(CommandToActionError::AttackCommandToActionError),
            Command::Examine(examine_cmd) => examine_action::from_command(examine_cmd, world)
                .map_err(CommandToActionError::ExamineCommandToActionError),
            Command::Circumspect => Ok(circumspect_action::action()),
            Command::Inventory => Ok(inventory_action::action(self.dummy())),
            Command::Wield(wield_cmd) => wield_action::from_command(self, wield_cmd)
                .map_err(CommandToActionError::WieldCommandToActionError),
            Command::Use(use_cmd) => use_action::from_command(self, use_cmd, world)
                .map_err(CommandToActionError::UseCommandToActionError),
        }
    }
}