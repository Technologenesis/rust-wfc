use std::error::Error as StdError;

use futures::future::BoxFuture;

use crate::worldobject::{
    fns::update::Action,
    components::{
        controllable::ControllableTrait,
        wielder::wieldable::Wieldable
    }
};
use crate::world::{World, handle::WorldObjectHandle};
use crate::quantities::direction::DirectionHorizontal;
use crate::lang::{VerbPhrase, TransitiveVerbPhrase, TransitiveVerb, verbs::ToWield};

use super::Human;

enum HumanUseWieldedItemError {
    FailedToFindWieldedItem(String),
    FailedToUseItem(Box<dyn std::error::Error>),
}

impl ControllableTrait for Human {
    fn wield(&mut self, item: Wieldable) -> Result<(), Box<dyn StdError>> {
        let inventory_item_description = item.indefinite_description();

        let wielding_arm = match self.dominant_arm {
            DirectionHorizontal::Left => {
                &mut self.body.torso.left_arm
            }
            DirectionHorizontal::Right => {
                &mut self.body.torso.right_arm
            }
        };

        wielding_arm.wield(item)?;

        Ok(Action{
            exec: Box::new(
                move |_: &mut World| -> BoxFuture<Result<Option<String>, Box<dyn StdError>>> {
                    Box::pin(async move {
                        Ok(None)
                    })
                }
            ),
            verb_phrase: VerbPhrase::Transitive(
                TransitiveVerbPhrase {
                    verb: TransitiveVerb::new(ToWield),
                    direct_object: inventory_item_description
                }
            )
        })
    }

    fn use_wielded_item(&self, world: &World, my_handle: WorldObjectHandle, item_name: String, targets: Vec<WorldObjectHandle>) -> Result<(), Box<dyn StdError>> {
        let wielded_item = self.wielded_items_mut().find(
            |item| item.name() == item_name
        ).ok_or(HumanUseWieldedItemError::FailedToFindWieldedItem(item_name))?;

        wielded_item.use_item(world, my_handle, targets)
    }

    fn attack(&mut self, target: WorldObjectHandle) -> Result<(), Box<dyn StdError>> {

    }
    
    fn examine(&mut self, target: WorldObjectHandle) -> Result<(), Box<dyn StdError>> {

    }
}