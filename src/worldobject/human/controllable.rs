use std::error::Error as StdError;

use crate::worldobject::{
    fns::update::Action,
    components::{
        controllable::ControllableTrait,
        controllable::controller::commands::Command,
        wielder::{WielderTrait, wieldable::Wieldable},
    }
};
use crate::world::{World, handle::WorldObjectHandle};
use crate::quantities::direction::DirectionHorizontal;

use super::Human;

impl ControllableTrait for Human {
    fn handle_command(&mut self, cmd: Command, _my_handle: &WorldObjectHandle, _world: &World) -> Result<Action, Box<dyn StdError>> {
        match cmd {
            Command::Use(use_cmd) => {
                let arm = match self.dominant_arm {
                    DirectionHorizontal::Left => &mut self.body.torso.left_arm,
                    DirectionHorizontal::Right => &mut self.body.torso.right_arm,
                };

                let hand = arm.hand.as_mut()
                    .ok_or("you don't have a hand to use items with")?;

                let item = hand.held_item.as_mut()
                    .ok_or("you are not wielding anything")?;

                if item.wieldable_name() != use_cmd.item_name {
                    return Err(Box::from(format!(
                        "you are not wielding an item named \"{}\"", use_cmd.item_name
                    )));
                }

                let usable = item.as_usable()
                    .ok_or_else(|| format!("the {} cannot be used", use_cmd.item_name))?;

                let effect = usable.use_item(use_cmd.target_handle.as_ref())?;
                let message = effect.message;

                Ok(Action {
                    exec: Box::new(move |_: &mut World| {
                        Box::pin(async move { Ok(Some(message)) })
                    }),
                    verb_phrase: effect.verb_phrase,
                })
            }

            Command::Move(_) => todo!("move via new component API"),
            Command::Interact(_) => todo!("interact via new component API"),
            Command::Collect(_) => todo!("collect via new component API"),
            Command::Attack(_) => todo!("attack via new component API"),
            Command::Examine(_) => todo!("examine via new component API"),
            Command::Wield(_) => todo!("wield via new component API"),
            Command::Circumspect => todo!("circumspect via new component API"),
            Command::Inventory => todo!("inventory via new component API"),
        }
    }

    fn wield(&mut self, item: Wieldable) -> Result<(), Box<dyn StdError>> {
        let wielding_arm = match self.dominant_arm {
            DirectionHorizontal::Left => &mut self.body.torso.left_arm,
            DirectionHorizontal::Right => &mut self.body.torso.right_arm,
        };
        wielding_arm.wield(item)
    }

    fn use_wielded_item(&mut self, item_name: String, targets: Vec<WorldObjectHandle>) -> Result<(), Box<dyn StdError>> {
        let arm = match self.dominant_arm {
            DirectionHorizontal::Left => &mut self.body.torso.left_arm,
            DirectionHorizontal::Right => &mut self.body.torso.right_arm,
        };

        let hand = arm.hand.as_mut()
            .ok_or("you don't have a hand to use items with")?;

        let item = hand.held_item.as_mut()
            .ok_or("you are not wielding anything")?;

        if item.wieldable_name() != item_name {
            return Err(Box::from(format!(
                "you are not wielding an item named \"{}\"", item_name
            )));
        }

        let usable = item.as_usable()
            .ok_or_else(|| format!("the {} cannot be used", item_name))?;

        usable.use_item(targets.first())?;
        Ok(())
    }

    fn attack(&mut self, _target: WorldObjectHandle) -> Result<(), Box<dyn StdError>> {
        todo!("attack via new component API")
    }

    fn examine(&mut self, _target: WorldObjectHandle) -> Result<(), Box<dyn StdError>> {
        todo!("examine via new component API")
    }
}
