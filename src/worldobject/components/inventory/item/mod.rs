pub mod none;

use crate::worldobject;
use crate::inventory;
use crate::quantities;
use crate::quantities::mass;
use crate::quantities::force;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct InventoryItemHandle(uuid::Uuid);

impl InventoryItemHandle {
    pub fn new() -> InventoryItemHandle {
        InventoryItemHandle(uuid::Uuid::new_v4())
    }
}

impl From<InventoryItemHandle> for String {
    fn from(handle: InventoryItemHandle) -> String {
        handle.0.to_string()
    }
}

pub trait InventoryItem: worldobject::WorldObject {
    fn dummy(&self) -> Box<dyn InventoryItem>;
}

impl worldobject::WorldObject for Box<dyn InventoryItem> {
    fn name(&self) -> String {
        (**self).name()
    }

    fn update(&mut self, my_handle: crate::world::WorldObjectHandle, world: &crate::world::World) -> Result<worldobject::UpdateFn, worldobject::Error> {
        (**self).update(my_handle, world)
    }

    fn examine(&self) -> String {
        (**self).examine()
    }

    fn dummy(&self) -> Box<dyn worldobject::WorldObject> {
        worldobject::WorldObject::dummy(&**self)
    }

    fn definite_description(&self) -> String {
        (**self).definite_description()
    }

    fn pronoun(&self) -> String {
        (**self).pronoun()
    }

    fn collect(self: Box<Self>) -> Result<Box<dyn InventoryItem>, (worldobject::Error, Box<dyn worldobject::WorldObject>)> {
        (*self).collect()
    }

    fn interact(&mut self) -> Result<String, worldobject::Error> {
        (**self).interact()
    }

    fn inventory(&self) -> Result<&inventory::Inventory, Box<dyn std::error::Error>> {
        (**self).inventory()
    }

    fn inventory_mut(&mut self) -> Result<&mut inventory::Inventory, Box<dyn std::error::Error>> {
        (**self).inventory_mut()
    }

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, Box<dyn std::error::Error>> {
        (**self).apply_force(force)
    }

    fn mass(&self) -> quantities::Quantity<mass::Mass> {
        (**self).mass()
    }

    fn send_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        (**self).send_message(message)
    }
}

impl InventoryItem for Box<dyn InventoryItem> {
    fn dummy(&self) -> Box<dyn InventoryItem> {
        InventoryItem::dummy(&(**self))
    }
}