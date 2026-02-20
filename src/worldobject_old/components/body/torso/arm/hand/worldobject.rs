use async_trait::async_trait;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    worldobject::components::inventory::{
        Inventory,
        item::InventoryItem
    },
    quantities::{
        Quantity,
        mass::Mass,
        force::Force
    },
    worldobject::{
        WorldObject,
        Error as WorldObjectError,
        fns::update::Action
    },
    world::{
        handle::WorldObjectHandle,
        World
    },
    worldobject::components::controllers::Controller
};

use super::Hand;
use super::inventoryitem;

#[derive(Debug)]
pub struct HandControllerError;

impl std::fmt::Display for HandControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandControllerError")
    }
}

impl std::error::Error for HandControllerError {}

#[derive(Debug)]
pub struct HandInventoryError;

impl Display for HandInventoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "HandInventoryError")
    }
}

impl std::error::Error for HandInventoryError {}

#[async_trait]
impl WorldObject for Hand {
    fn name(&self) -> String {
        String::from("hand")
    }

    fn definite_description(&self) -> String {
        String::from("the hand")
    }

    fn indefinite_description(&self) -> String {
        String::from("a hand")
    }

    fn pronoun(&self) -> String {
        String::from("it")
    }

    fn dummy(&self) -> Box<dyn WorldObject> {
        Box::new(Hand {
            held_item: self.held_item.as_ref().map(
                |held_item| InventoryItem::dummy(&*held_item)
            ),
            base_mass: self.base_mass.clone()
        })
    }

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, WorldObjectError> {
        Ok(Action::no_op())
    }

    async fn collect(self: Box<Self>) -> Result<Box<dyn InventoryItem>, (WorldObjectError, Box<dyn WorldObject>)> {
        Ok(self)
    }

    fn inventory(&self) -> Result<&Inventory, WorldObjectError> {
        Err(Box::new(HandInventoryError {}))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, WorldObjectError> {
        Err(Box::new(HandInventoryError {}))
    }

    fn mass(&self) -> Quantity<Mass> {
        let mut ret = self.base_mass.clone();

        if let Some(held_item) = &self.held_item {
            ret = ret + held_item.mass();
        }

        ret
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, WorldObjectError> {
        Ok(String::from(""))
    }
    
    fn examine(&self) -> String {
        String::from("an arm")
    }

    async fn send_message(&mut self, message: String) -> Result<(), WorldObjectError> {
        Ok(())
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        Ok(String::from("you can't think of anything particularly interesting to do with this."))
    }

    fn controller(&self) -> Result<&dyn Controller, WorldObjectError> {
        Err(Box::new(HandControllerError))
    }

    fn controller_mut(&mut self) -> Result<&mut dyn Controller, WorldObjectError> {
        Err(Box::new(HandControllerError))
    }
    
    fn take_controller(&mut self) -> Result<Box<dyn Controller>, WorldObjectError> {
        Err(Box::new(HandControllerError))
    }

    fn set_controller(&mut self, controller: Box<dyn Controller>) -> Result<(), (Box<dyn Controller>, WorldObjectError)> {
        Err((controller, Box::new(HandControllerError)))
    }
}
