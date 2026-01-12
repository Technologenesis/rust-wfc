use std::collections::HashMap;

use crate::worldobject::TypedWorldObject;
use crate::inventory::item::{InventoryItem, InventoryItemHandle};

use uuid;

use serde::ser::SerializeStruct;

pub mod item;

pub struct Inventory(pub HashMap<item::InventoryItemHandle, Box<dyn InventoryItem>>);

impl<'de> serde::Deserialize<'de> for Inventory {
    fn deserialize<D: serde::Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        let inventory = Inventory(HashMap::new());
        Ok(inventory)
    }
}

impl serde::Serialize for Inventory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let state = serializer.serialize_struct("Inventory", 0)?;
        state.end()
    }
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory(HashMap::new())
    }

    pub fn give<Item: InventoryItem + 'static>(&mut self, item: Item) -> InventoryItemHandle {
        let handle = InventoryItemHandle::new();

        self.0.insert(handle.clone(), Box::new(item));

        handle
    }

    pub fn dummy(&self) -> Self {
        Self(self.0.iter().map(
            |(handle, item)| (handle.clone(), item.dummy())
        ).collect())
    }
}