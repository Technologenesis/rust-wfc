pub mod human;
pub mod rat;
pub mod none;
pub mod components;
pub mod fns;
pub mod new;

use async_trait::async_trait;

use crate::{
    world::{
        World,
        handle::WorldObjectHandle
    },
    worldobject::components::{
        controllers::Controller,
        inventory::{
            Inventory,
            item::InventoryItem
        }
    },
    quantities::{
        Quantity,
        mass::Mass,
        force::Force
    }
};

use fns::update::Action;

type Error = Box<dyn std::error::Error>;

#[async_trait]
pub trait WorldObject: Send + Sync {
    // linguistic accessors
    fn name(&self) -> String;
    fn examine(&self) -> String;
    fn definite_description(&self) -> String;
    fn indefinite_description(&self) -> String;
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

    // controller accessors
    fn controller(&self) -> Result<&dyn Controller, Error>;
    fn controller_mut(&mut self) -> Result<&mut dyn Controller, Error>;
    fn take_controller(&mut self) -> Result<Box<dyn Controller>, Error>;
    fn set_controller(&mut self, controller: Box<dyn Controller>) -> Result<(), (Box<dyn Controller>, Error)>;
}