use async_trait::async_trait;

use crate::{
    world::{
        handle::WorldObjectHandle,
        World
    },
    worldobject::{
        components::controllers::Controller,
        none,
        WorldObject,
        TypedWorldObject,
        fns::update::Action,
        Error as WorldObjectError
    },
    quantities::{
        Quantity,
        mass::Mass,
        force::Force
    }
};

use super::{
    InventoryItem,
    super::Inventory
};

pub struct NoInventoryItem(pub none::NoWorldObject);

#[async_trait]
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

    fn indefinite_description(&self) -> String {
        WorldObject::indefinite_description(&self.0)
    }

    fn pronoun(&self) -> String {
        WorldObject::pronoun(&self.0)
    }

    fn dummy(&self) -> Self::Dummy {
        Self(TypedWorldObject::dummy(&self.0))
    }

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, WorldObjectError> {
        WorldObject::update(&mut self.0, my_handle, world).await
    }

    async fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (WorldObjectError, Box<Self>)> {
        Ok(*self)
    }
    
    fn inventory(&self) -> Result<&Inventory, WorldObjectError> {
        WorldObject::inventory(&self.0)
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, WorldObjectError> {
        WorldObject::inventory_mut(&mut self.0)
    }
    
    fn mass(&self) -> Quantity<Mass> {
        WorldObject::mass(&self.0)
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, WorldObjectError> {
        WorldObject::apply_force(&mut self.0, force).await
    }
    
    
    async fn send_message(&mut self, message: String) -> Result<(), WorldObjectError> {
        WorldObject::send_message(&mut self.0, message).await
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        WorldObject::interact(&mut self.0).await
    }

    fn controller(&self) -> Result<&dyn Controller, WorldObjectError> {
        WorldObject::controller(&self.0)
    }

    fn controller_mut(&mut self) -> Result<&mut dyn Controller, WorldObjectError> {
        WorldObject::controller_mut(&mut self.0)
    }

    fn take_controller(&mut self) -> Result<Box<dyn Controller>, WorldObjectError> {
        WorldObject::take_controller(&mut self.0)
    }

    fn set_controller<C: Controller + 'static>(&mut self, controller: C) -> Result<(), (C, WorldObjectError)> {
        TypedWorldObject::set_controller(&mut self.0, controller)
    }
}

#[derive(Debug)]
pub struct NoInventoryItemUseError;

impl std::error::Error for NoInventoryItemUseError {}

impl std::fmt::Display for NoInventoryItemUseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "how did you even get one of these?")
    }
}

impl InventoryItem for NoInventoryItem {
    fn dummy(&self) -> Box<dyn InventoryItem> {
        Box::new(NoInventoryItem(TypedWorldObject::dummy(&self.0)))
    }

    fn use_item(&mut self, _: &World, _: WorldObjectHandle, _: Option<WorldObjectHandle>) -> Result<Action, Box<dyn std::error::Error>> {
        return Err(Box::new(NoInventoryItemUseError));
    }
}