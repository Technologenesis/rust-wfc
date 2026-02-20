use std::error::Error as StdError;

use crate::worldobject::{
    components::{
        controllable::ControllableTrait,
        wielder::{WielderTrait, wieldable::Wieldable},
    }
};
use crate::world::handle::WorldObjectHandle;
use crate::quantities::direction::DirectionHorizontal;

use super::Human;

impl ControllableTrait for Human {
    fn wield(&mut self, item: Wieldable) -> Result<(), Box<dyn StdError>> {
        let wielding_arm = match self.dominant_arm {
            DirectionHorizontal::Left => &mut self.body.torso.left_arm,
            DirectionHorizontal::Right => &mut self.body.torso.right_arm,
        };
        wielding_arm.wield(item)
    }

    fn use_wielded_item(&mut self, _item_name: String, _targets: Vec<WorldObjectHandle>) -> Result<(), Box<dyn StdError>> {
        todo!("use wielded item via new component API")
    }

    fn attack(&mut self, _target: WorldObjectHandle) -> Result<(), Box<dyn StdError>> {
        todo!("attack via new component API")
    }

    fn examine(&mut self, _target: WorldObjectHandle) -> Result<(), Box<dyn StdError>> {
        todo!("examine via new component API")
    }
}
