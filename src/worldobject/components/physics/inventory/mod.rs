pub mod item;

use std::collections::HashMap;

use item::{
    InventoryItem,
    InventoryItemHandle
};


pub struct InventoryComponent(pub HashMap<item::InventoryItemHandle, Box<dyn InventoryItem>>);

impl<'de> serde::Deserialize<'de> for InventoryComponent {
    fn deserialize<D: serde::Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        let inventory = InventoryComponent(HashMap::new());
        Ok(inventory)
    }
}

impl serde::Serialize for InventoryComponent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }
}

impl InventoryComponent {
    pub fn new() -> InventoryComponent {
        InventoryComponent(HashMap::new())
    }

    pub fn give<Item: InventoryItem + 'static>(&mut self, item: Item) -> InventoryItemHandle {
        let handle = InventoryItemHandle::new();

        self.0.insert(handle.clone(), Box::new(item));

        handle
    }

    pub fn get(&self, handle: &InventoryItemHandle) -> Option<&Box<dyn InventoryItem>> {
        self.0.get(handle)
    }

    pub fn get_mut(&mut self, handle: &InventoryItemHandle) -> Option<&mut Box<dyn InventoryItem>> {
        self.0.get_mut(handle)
    }

    pub fn take(&mut self, handle: &InventoryItemHandle) -> Option<Box<dyn InventoryItem>> {
        self.0.remove(handle)
    }

    pub fn dummy(&self) -> Self {
        Self(self.0.iter().map(
            |(handle, item)| (handle.clone(), item.dummy())
        ).collect())
    }
}