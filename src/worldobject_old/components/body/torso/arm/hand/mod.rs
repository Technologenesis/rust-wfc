mod worldobject;
mod inventoryitem;

use serde::ser::SerializeStruct;

use crate::{
    worldobject::components::inventory::item::InventoryItem,
    quantities::{Quantity, mass::Mass}
};
use std::fmt;

pub struct Hand {
    base_mass: Quantity<Mass>,
    held_item: Option<Box<dyn InventoryItem>>,
}

pub struct HandDeserializeError;

impl fmt::Display for HandDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandDeserializeError")
    }
}

impl<'de> TryFrom<&serde_json::Value> for Hand {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let base_mass = value.get("base_mass").ok_or(String::from("base_mass not found"))
            .and_then(|v| Quantity::<Mass>::try_from(v.clone()).map_err(|err| format!("failed to parse base_mass: {}", err)))?;
        
        Ok(Hand {
            held_item: None,
            base_mass: base_mass,
        })
    }
}

impl serde::Serialize for Hand {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Hand", 1)?;
        state.serialize_field("base_mass", &self.base_mass)?;
        state.end()
    }
}

pub fn hand(base_mass: Quantity<Mass>, held_item: Option<impl InventoryItem + 'static>) -> Hand {
    Hand {
        held_item: held_item.map(|item| {
            let item: Box<dyn InventoryItem> = Box::new(item);
            item
        }),
        base_mass,
    }
}

impl Hand {
    pub fn wield(&mut self, item: Box<dyn InventoryItem>) {
        self.held_item = Some(item);
    }

    pub fn wielded_item(&self) -> Option<&dyn InventoryItem> {
        self.held_item.as_ref().map(|item| item.as_ref())
    }

    pub fn wielded_item_mut(&mut self) -> Option<&mut dyn InventoryItem> {
        self.held_item.as_mut().map(|item| {
            item.as_mut()
        })
    }

    pub fn dummy(&self) -> Self {
        Hand {
            held_item: self.held_item.as_ref().map(
                |held_item| InventoryItem::dummy(&*held_item)
            ),
            base_mass: self.base_mass.clone()
        }
    }
}