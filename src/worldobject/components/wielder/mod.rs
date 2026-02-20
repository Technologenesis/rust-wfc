pub mod wieldable;

use std::error::Error as StdError;

use crate::worldobject::WorldObject;

use wieldable::Wieldable;

pub type Wielder = Box<dyn WielderTrait>;

pub trait WielderTrait: WorldObject {
    fn wield(&mut self, item: Wieldable) -> Result<(), Box<dyn StdError>>;
}