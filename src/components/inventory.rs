use std::collections::HashMap;

use crate::world;
use crate::worldobject;
use crate::quantities::mass;
use crate::quantities::force;
use crate::quantities;
use crate::worldobject::TypedWorldObject;
use uuid;

use serde::ser::SerializeStruct;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct InventoryItemHandle(uuid::Uuid);

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

    fn inventory(&self) -> Result<&Inventory, Box<dyn std::error::Error>> {
        (**self).inventory()
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, Box<dyn std::error::Error>> {
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

pub struct Inventory(pub HashMap<InventoryItemHandle, Box<dyn InventoryItem>>);

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
        let handle = InventoryItemHandle(uuid::Uuid::new_v4());

        self.0.insert(handle.clone(), Box::new(item));

        handle
    }

    pub fn dummy(&self) -> Self {
        Self(self.0.iter().map(
            |(handle, item)| (handle.clone(), item.dummy())
        ).collect())
    }
}

pub struct NoInventoryItem(pub worldobject::NoWorldObject);

impl worldobject::TypedWorldObject for NoInventoryItem {
    type Dummy = Self;
    type CollectInventoryItem = Self;

    fn name(&self) -> String {
        self.0.name()
    }

    fn examine(&self) -> String {
        self.0.examine()
    }

    fn definite_description(&self) -> String {
        self.0.definite_description()
    }

    fn pronoun(&self) -> String {
        self.0.pronoun()
    }

    fn dummy(&self) -> Box<dyn worldobject::WorldObject> {
        Box::new(Self(self.0))
    }

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<worldobject::UpdateFn, worldobject::Error> {
        self.0.update(my_handle, world)
    }

    fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (worldobject::Error, Box<Self>)> {
        Ok(self)
    }
    
    fn inventory(&self) -> Result<&Inventory, worldobject::Error> {
        Err(Box::new(NoInventoryError()))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, worldobject::Error> {
        Err(Box::new(NoInventoryError()))
    }
    
    fn mass(&self) -> quantities::Quantity<mass::Mass> {
        quantities::Quantity::zero()
    }

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, worldobject::Error> {
        Ok(String::from("nothing happens"))
    }
    
    
    fn send_message(&mut self, message: String) -> Result<(), worldobject::Error> {
        Ok(())
    }

    fn interact(&mut self) -> Result<String, worldobject::Error> {
        Ok(String::from("nothing happens"))
    }
}

impl InventoryItem for NoInventoryItem {
    fn dummy(&self) -> Box<dyn InventoryItem> {

        Box::new(NoInventoryItem(self.0))
    }
}