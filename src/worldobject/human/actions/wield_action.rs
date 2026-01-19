use futures::future::BoxFuture;

use crate::{
    lang::{TransitiveVerb, TransitiveVerbPhrase, VerbPhrase, verbs::ToWield}, world::World, worldobject::{
        Error as WorldObjectError, TypedWorldObject, components::{
            controllers::commands::wield_command::WieldCommand,
            inventory::item::InventoryItemHandle
        }, fns::update::Action, human::Human
    },
    quantities::direction::DirectionHorizontal,
};

#[derive(Debug)]
pub enum WieldCommandToActionError {
    NoSuchItem(InventoryItemHandle),
}

impl std::fmt::Display for WieldCommandToActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchItem(handle) => write!(f, "inventory does not contain item: {}", handle),
        }
    }
}

impl std::error::Error for WieldCommandToActionError {}

pub fn from_command(me: &mut Human, cmd: WieldCommand) -> Result<Action, WieldCommandToActionError> {
    let inventory_item = me.unsouled.inventory.take(&cmd.item_handle)
        .ok_or(WieldCommandToActionError::NoSuchItem(cmd.item_handle))?;
    let inventory_item_description = inventory_item.definite_description();

    let wielding_arm = match me.unsouled.dominant_arm {
        DirectionHorizontal::Left => {
            &mut me.unsouled.body.torso.left_arm
        }
        DirectionHorizontal::Right => {
            &mut me.unsouled.body.torso.right_arm
        }
    };

    wielding_arm.wield(inventory_item);

    Ok(Action{
        exec: Box::new(
            move |_: &mut World| -> BoxFuture<Result<Option<String>, WorldObjectError>> {
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