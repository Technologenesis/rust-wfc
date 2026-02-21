pub mod controller;

use std::error::Error as StdError;

use crate::world::{World, handle::WorldObjectHandle};
use crate::worldobject::{
    WorldObject,
    fns::update::Action,
    components::wielder::wieldable::Wieldable,
};

use controller::commands::Command;

pub type Controllable = Box<dyn ControllableTrait>;

pub trait ControllableTrait: WorldObject {
    fn handle_command(&mut self, cmd: Command, my_handle: &WorldObjectHandle, world: &World) -> Result<Action, Box<dyn StdError>>;

    fn wield(&mut self, item: Wieldable) -> Result<(), Box<dyn StdError>>;
    fn use_wielded_item(&mut self, item_name: String, targets: Vec<WorldObjectHandle>) -> Result<(), Box<dyn StdError>>;
    fn attack(&mut self, target: WorldObjectHandle) -> Result<(), Box<dyn StdError>>;
    fn examine(&mut self, target: WorldObjectHandle) -> Result<(), Box<dyn StdError>>;
}