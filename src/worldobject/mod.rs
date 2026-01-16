pub mod human;
pub mod rat;
pub mod sword;
pub mod none;
pub mod components;
pub mod fns;

use async_trait::async_trait;

use crate::{
    world::{
        World,
        handle::WorldObjectHandle
    },
    worldobject::components::inventory::{
        Inventory,
        item::InventoryItem
    },
    quantities::{
        Quantity,
        mass::Mass,
        force::Force
    }
};

use fns::update::Action;

type Error = Box<dyn std::error::Error>;

// TypedWorldObject is a trait similar to WorldObject,
// but with type parameters where WorldObject uses dyn
// trait objects.  Implementing this trait automatically
// implements the WorldObject trait, but may allow certain
// users of the type to rely on more specific type constraints.
#[async_trait]
pub trait TypedWorldObject: Send {
    type Dummy: WorldObject + Sized + 'static;
    type CollectInventoryItem: InventoryItem + Sized + 'static;

    // linguistic accessors
    fn name(&self) -> String;
    fn examine(&self) -> String;
    fn definite_description(&self) -> String;
    fn pronoun(&self) -> String;

    // creates a new object with the same properties as this one,
    // minus any fields that are not cloneable (e.g. controllers,
    // loggers)
    fn dummy(&self) -> Self::Dummy;

    // inventory accessors
    fn inventory(&self) -> Result<&Inventory, Error>;
    fn inventory_mut(&mut self) -> Result<&mut Inventory, Error>;

    // mass accessor
    fn mass(&self) -> Quantity<Mass>;

    // game mechanics; all async to allow interaction with the controller.
    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, Error>;
    async fn send_message(&mut self, message: String) -> Result<(), Error>;
    async fn interact(&mut self) -> Result<String, Error>;
    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, Error>;
    async fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (Error, Box<Self>)>;
}

#[async_trait]
impl<T: TypedWorldObject + Send + Sync + 'static> WorldObject for T {
    fn name(&self) -> String {
        <T as TypedWorldObject>::name(self)
    }

    fn examine(&self) -> String {
        <T as TypedWorldObject>::examine(self)
    }

    fn definite_description(&self) -> String {
        <T as TypedWorldObject>::definite_description(&self)
    }

    fn pronoun(&self) -> String {
        <T as TypedWorldObject>::pronoun(&self)
    }

    fn dummy(&self) -> Box<dyn WorldObject> {
        Box::new(<T as TypedWorldObject>::dummy(self))
    }

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, Error> {
        <T as TypedWorldObject>::update(self, my_handle, world).await
    }

    async fn collect(self: Box<Self>) -> Result<Box<dyn InventoryItem>, (Error, Box<dyn WorldObject>)> {
        <T as TypedWorldObject>::collect(self).await
            .map(|item| Box::new(item) as Box<dyn InventoryItem>)
            .map_err(|(err, obj)| (err, obj as Box<dyn WorldObject>))
    }

    fn inventory(&self) -> Result<&Inventory, Error> {
        <T as TypedWorldObject>::inventory(&self)
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, Error> {
        <T as TypedWorldObject>::inventory_mut(self)
    }

    fn mass(&self) -> Quantity<Mass> {
        <T as TypedWorldObject>::mass(self)
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, Error> {
        <T as TypedWorldObject>::apply_force(self, force).await
    }

    async fn send_message(&mut self, message: String) -> Result<(), Error> {
        <T as TypedWorldObject>::send_message(self, message).await
    }

    async fn interact(&mut self) -> Result<String, Error> {
        <T as TypedWorldObject>::interact(self).await
    }
}

#[async_trait]
pub trait WorldObject: Send + Sync {
    // linguistic accessors
    fn name(&self) -> String;
    fn examine(&self) -> String;
    fn definite_description(&self) -> String;
    fn pronoun(&self) -> String;

    // physics
    fn mass(&self) -> Quantity<Mass>;

    // inventory accessors
    fn inventory(&self) -> Result<&Inventory, Error>;
    fn inventory_mut(&mut self) -> Result<&mut Inventory, Error>;

    // creates a new object with the same properties as this one,
    // minus any fields that are not cloneable (typically controllers)
    fn dummy(&self) -> Box<dyn WorldObject>;

    // game mechanics; all async to allow interaction with the controller.
    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, Error>;
    async fn collect(self: Box<Self>) -> Result<Box<dyn InventoryItem>, (Error, Box<dyn WorldObject>)>;
    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, Error>;
    async fn send_message(&mut self, message: String) -> Result<(), Error>;
    async fn interact(&mut self) -> Result<String, Error>;
}