pub mod unsouled;
pub mod actions;

use std::error::Error as StdError;
use async_trait::async_trait;

use crate::{
    quantities::{
        Quantity, direction::DirectionHorizontal, force::Force, mass::Mass
    }, world::{
        World,
        handle::WorldObjectHandle
    }, worldobject::{
        Error as WorldObjectError, TypedWorldObject, components::{
            controllers::Controller,
            gender::Gender,
            body::Body,
            inventory::{
                Inventory,
                item::none::NoInventoryItem
            }
        }, fns::update::Action
    },
    worldobject::human::actions as human_actions
};

use unsouled::{
    UnsouledHuman,
};

pub struct Human {
    unsouled: UnsouledHuman,
    controller: Box<dyn Controller>,
}

impl Human {
    pub fn new(
        unsouled: UnsouledHuman,
        controller: impl Controller + 'static,
    ) -> Human {
        Human{
            unsouled: unsouled,
            controller: Box::new(controller),
        }
    }

    pub fn desouled(self) -> (UnsouledHuman, Box<dyn Controller>) {
        (self.unsouled, self.controller)
    }
}

#[async_trait]
impl TypedWorldObject for Human {
    type Dummy = UnsouledHuman;
    // humans can't be collected
    type CollectInventoryItem = NoInventoryItem;

    fn name(&self) -> String {
        self.unsouled.name.clone()
    }

    fn definite_description(&self) -> String {
        self.unsouled.name.clone()
    }

    fn pronoun(&self) -> String {
        self.unsouled.gender.subject_pronoun().to_string()
    }

    async fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (WorldObjectError, Box<Self>)> {
        Err((Box::new(unsouled::HumanCollectError()), self))
    }

    fn dummy(&self) -> Self::Dummy {
        self.unsouled.dummy()
    }

    fn examine(&self) -> String {
        format!("a human {}; {} name is {}", self.unsouled.gender.noun(), self.unsouled.gender.possessive_pronoun(), self.unsouled.name)
    }

    fn inventory(&self) -> Result<&Inventory, WorldObjectError> {
        Ok(&self.unsouled.inventory)
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, WorldObjectError> {
        Ok(&mut self.unsouled.inventory)
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        Ok(format!("{} says \"Hello\".", self.unsouled.name))
    }

    /*
    fn interact(&self, actor: &world::WorldObjectHandle, world: &mut world::World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    */

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, WorldObjectError> {
        let action = self.controller.prompt_turn().await?;

        human_actions::from_command(action, world, my_handle, self.dummy())
            .map_err(|err| Box::new(err).into())
    }

    async fn send_message(&mut self, message: String) -> Result<(), WorldObjectError> {
        self.controller.display_message(message).await?;
        Ok(())
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, WorldObjectError> {
        self.unsouled.apply_force(force).await
    }

    fn mass(&self) -> Quantity<Mass> {
        self.unsouled.mass()
    }
}
