use crate::components::inventory;
use crate::world::WorldObjectSendMessageError;
use crate::worldobject;
use crate::world;

use serde::Serialize;
use serde::Deserialize;

use serde::ser::SerializeStruct;

pub struct Hand {
    held_item: Option<Box<dyn inventory::InventoryItem>>,
}

impl worldobject::TypedWorldObject for Hand {
    type Dummy = Self;
    type CollectInventoryItem = Self;

    fn name(&self) -> String {
        String::from("hand")
    }

    fn definite_description(&self) -> String {
        String::from("a hand")
    }

    fn pronoun(&self) -> String {
        String::from("it")
    }

    fn dummy(&self) -> Self::Dummy {
        Hand {
            held_item: self.held_item.map(
                |held_item| <Box<dyn inventory::InventoryItem> as worldobject::TypedWorldObject>::dummy(&held_item)
            )
        }
    }

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<worldobject::UpdateFn, worldobject::Error> {
        Ok(worldobject::UpdateFn::no_op())
    }

    fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (worldobject::Error, Self)> {
        Ok(*self)
    }

    fn inventory(&self) -> Result<&inventory::Inventory, worldobject::Error> {
        Err(Box::new(HandInventoryError {}))
    }

    fn inventory_mut(&mut self) -> Result<&mut inventory::Inventory, worldobject::Error> {
        Err(Box::new(HandInventoryError {}))
    }

    fn mass(&self) -> quantities::Quantity<quantities::mass::Mass> {
        self.hand.as_ref().map(
            |hand| &self.base_mass + &<hand::Hand as WorldObject>::mass(hand)
        ).unwrap_or(self.base_mass.clone())
    }

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, worldobject::Error> {
        Ok(String::from(""))
    }
    
    fn examine(&self) -> String {
        String::from("an arm")
    }

    fn send_message(&mut self, message: String) -> Result<(), worldobject::Error> {
        Ok(())
    }

    fn interact(&mut self) -> Result<String, worldobject::Error> {
        Ok(String::from("you can't think of anything particularly interesting to do with this."))
    }
}

impl inventory::InventoryItem for Hand {
    fn dummy(&self) -> Box<dyn inventory::InventoryItem> {
        Box::new(<Hand as worldobject::TypedWorldObject>::dummy(self))
    }
}

impl<'de> serde::Deserialize<'de> for Hand {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Hand { held_item: None })
    }
}

impl serde::Serialize for Hand {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let state = serializer.serialize_struct("Hand", 0)?;
        state.end()
    }
}

pub fn hand(held_item: Option<impl inventory::InventoryItem + 'static>) -> Hand {
    Hand {
        held_item: held_item.map(|item| {
            let item: Box<dyn inventory::InventoryItem> = Box::new(item);
            item
        })
    }
}