pub mod controller;

use std::error::Error as StdError;

use crate::world::handle::WorldObjectHandle;
use crate::worldobject::{WorldObject, components::wielder::wieldable::Wieldable};

pub type Controllable = Box<dyn ControllableTrait>;

pub trait ControllableTrait: WorldObject {
    fn wield(&mut self, item: Wieldable) -> Result<(), Box<dyn StdError>>;
    fn use_wielded_item(&mut self, item_name: String, targets: Vec<WorldObjectHandle>) -> Result<(), Box<dyn StdError>>;
    fn attack(&mut self, target: WorldObjectHandle) -> Result<(), Box<dyn StdError>>;
    fn examine(&mut self, target: WorldObjectHandle) -> Result<(), Box<dyn StdError>>;
}