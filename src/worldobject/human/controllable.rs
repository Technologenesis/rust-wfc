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

    fn use_wielded_item(&mut self, item_name: String, _targets: Vec<WorldObjectHandle>) -> Result<(), Box<dyn StdError>> {
        let arm = match self.dominant_arm {
            DirectionHorizontal::Left => &mut self.body.torso.left_arm,
            DirectionHorizontal::Right => &mut self.body.torso.right_arm,
        };

        let hand = arm.hand.as_mut()
            .ok_or_else(|| -> Box<dyn StdError> { Box::from("you don't have a hand to use items with") })?;

        let item = hand.held_item.as_mut()
            .ok_or_else(|| -> Box<dyn StdError> { Box::from("you are not wielding anything") })?;

        if item.wieldable_name() != item_name {
            return Err(Box::from(format!("you are not wielding an item named \"{}\"", item_name)));
        }

        let usable = item.as_usable()
            .ok_or_else(|| -> Box<dyn StdError> { Box::from(format!("the {} cannot be used", item_name)) })?;

        let target = _targets.first();
        usable.use_item(target)?;
        Ok(())
    }

    fn attack(&mut self, _target: WorldObjectHandle) -> Result<(), Box<dyn StdError>> {
        todo!("attack via new component API")
    }

    fn examine(&mut self, _target: WorldObjectHandle) -> Result<(), Box<dyn StdError>> {
        todo!("examine via new component API")
    }
}
