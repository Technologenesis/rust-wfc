use crate::world;
use crate::inventory;
use crate::quantities;
use crate::quantities::mass;
use crate::quantities::force;
use crate::{
    worldobject::{
        none,
        WorldObject,
        TypedWorldObject,
        UpdateFn,
        Error
    }
};


pub struct NoInventoryItem(pub none::NoWorldObject);

impl TypedWorldObject for NoInventoryItem {
    type Dummy = Self;
    type CollectInventoryItem = Self;

    fn name(&self) -> String {
        WorldObject::name(&self.0)
    }

    fn examine(&self) -> String {
        WorldObject::examine(&self.0)
    }

    fn definite_description(&self) -> String {
        WorldObject::definite_description(&self.0)
    }

    fn pronoun(&self) -> String {
        WorldObject::pronoun(&self.0)
    }

    fn dummy(&self) -> Self::Dummy {
        Self(TypedWorldObject::dummy(&self.0))
    }

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<UpdateFn, Error> {
        WorldObject::update(&mut self.0, my_handle, world)
    }

    fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (Error, Box<Self>)> {
        Ok(*self)
    }
    
    fn inventory(&self) -> Result<&inventory::Inventory, Error> {
        WorldObject::inventory(&self.0)
    }

    fn inventory_mut(&mut self) -> Result<&mut inventory::Inventory, Error> {
        WorldObject::inventory_mut(&mut self.0)
    }
    
    fn mass(&self) -> quantities::Quantity<mass::Mass> {
        WorldObject::mass(&self.0)
    }

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, Error> {
        WorldObject::apply_force(&mut self.0, force)
    }
    
    
    fn send_message(&mut self, message: String) -> Result<(), Error> {
        WorldObject::send_message(&mut self.0, message)
    }

    fn interact(&mut self) -> Result<String, Error> {
        WorldObject::interact(&mut self.0)
    }
}

impl inventory::InventoryItem for NoInventoryItem {
    fn dummy(&self) -> Box<dyn inventory::InventoryItem> {
        Box::new(NoInventoryItem(TypedWorldObject::dummy(&self.0)))
    }
}