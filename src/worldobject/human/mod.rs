pub mod actions;

use std::error::Error as StdError;
use async_trait::async_trait;
use serde::{Serialize, Serializer, ser::SerializeStruct};

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
                item::{InventoryItem, none::NoInventoryItem}
            }
        }, fns::update::Action
    }
};

pub struct Human {
    // identity
    pub name: String,
    pub gender: Gender,

    // misc
    pub dominant_arm: DirectionHorizontal,

    // body
    pub body: Body,
    pub inventory: Inventory,

    // controller
    controller: Option<Box<dyn Controller>>
}

#[derive(Debug)]
pub struct HumanCollectError ();

impl std::fmt::Display for HumanCollectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "humans can't be collected")
    }
}

impl StdError for HumanCollectError {}

#[derive(Debug)]
pub struct HumanNoControllerError ();

impl std::fmt::Display for HumanNoControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "human has no controller")
    }
}

impl StdError for HumanNoControllerError {}

impl Human {
    pub fn new(
        name: String,
        gender: Gender,
        body: Body,
        dominant_arm: DirectionHorizontal,
        inventory: Inventory,
        controller: Option<impl Controller + 'static>,
    ) -> Human {
        Human{
            name,
            gender,
            body,
            dominant_arm,
            inventory,
            controller: controller.map(
                |controller| -> Box<dyn Controller> {
                    Box::new(controller)
                }
            ),
        }
    }
}

#[async_trait]
impl TypedWorldObject for Human {
    type Dummy = Human;
    // humans can't be collected
    type CollectInventoryItem = NoInventoryItem;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn definite_description(&self) -> String {
        self.name.clone()
    }

    fn indefinite_description(&self) -> String {
        format!("a {}", self.gender.noun())
    }

    fn pronoun(&self) -> String {
        self.gender.subject_pronoun().to_string()
    }

    async fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (WorldObjectError, Box<Self>)> {
        Err((Box::new(HumanCollectError()), self))
    }

    fn dummy(&self) -> Self::Dummy {
        Human {
            name: self.name.clone(),
            gender: self.gender.clone(),
            body: self.body.dummy(),
            dominant_arm: self.dominant_arm.clone(),
            inventory: self.inventory.dummy(),
            controller: None,
        }
    }

    fn examine(&self) -> String {
        format!("a human {}; {} name is {}", self.gender.noun(), self.gender.possessive_pronoun(), self.name)
    }

    fn inventory(&self) -> Result<&Inventory, WorldObjectError> {
        Ok(&self.inventory)
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, WorldObjectError> {
        Ok(&mut self.inventory)
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        Ok(format!("{} says \"Hello\".", self.name))
    }

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, WorldObjectError> {
        match &mut self.controller {
            Some(controller) => {
                let command = controller.prompt_turn().await?;
                self.from_command(command, world, my_handle)
                    .map_err(|err| Box::new(err).into())
            }
            None => Ok(Action::no_op()),
        }
    }

    async fn send_message(&mut self, message: String) -> Result<(), WorldObjectError> {
        match &mut self.controller {
            Some(controller) => controller.display_message(message).await,
            None => Ok(()),
        }
    }

    async fn apply_force(&mut self, _: &Quantity<Force>) -> Result<String, WorldObjectError> {
        Ok(format!("{}'s hefty constitution absorbs the force.", self.definite_description()))
    }

    fn mass(&self) -> Quantity<Mass> {
        self.body.mass()
    }

    fn controller(&self) -> Result<&dyn Controller, Box<dyn StdError>> {
        match &self.controller {
            Some(controller) => Ok(&**controller),
            None => Err(Box::new(HumanNoControllerError())),
        }
    }

    fn controller_mut(&mut self) -> Result<&mut dyn Controller, Box<dyn StdError>> {
        match &mut self.controller {
            Some(controller) => Ok(&mut **controller),
            None => Err(Box::new(HumanNoControllerError())),
        }
    }

    fn take_controller(&mut self) -> Result<Box<dyn Controller>, Box<dyn StdError>> {
        let mut new_controller = Option::None::<Box<dyn Controller>>;
        std::mem::swap(&mut self.controller, &mut new_controller);
        new_controller.ok_or(Box::new(HumanNoControllerError()))
    }

    fn set_controller<C: Controller + 'static>(&mut self, controller: C) -> Result<(), (C, Box<dyn StdError>)> {
        self.controller = Some(Box::new(controller));
        Ok(())
    }
}

impl Human {
    pub fn wielded_items<'a>(&'a self) -> impl Iterator<Item = &'a dyn InventoryItem> {
        let left_items = self.body.torso.left_arm.wielded_item().into_iter();
        let right_items = self.body.torso.right_arm.wielded_item().into_iter();
        left_items.chain(right_items)
    }

    pub fn wielded_items_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut dyn InventoryItem> {
        let left_items = self.body.torso.left_arm.wielded_item_mut().into_iter();
        let right_items = self.body.torso.right_arm.wielded_item_mut().into_iter();
        left_items.chain(right_items)
    }
}

impl Serialize for Human {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Human", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("gender", &self.gender)?;
        state.serialize_field("body", &self.body)?;
        state.serialize_field("dominant_arm", &self.dominant_arm)?;
        state.serialize_field("inventory", &self.inventory)?;
        state.end()
    }
}



impl TryFrom<&serde_json::Value> for Human {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let name = value.get("name").and_then(|v| v.as_str()).ok_or("name not found")?;
        let gender = value.get("gender").map(|v| Gender::try_from(v)).transpose().map_err(|_| "gender not found")?.ok_or("gender not found")?;

        let dominant_arm = value.get("dominant_arm").map(|v| DirectionHorizontal::try_from(v)).transpose().map_err(|err| format!("failed to parse dominant_arm: {}", err))?.ok_or("dominant_arm not found")?;

        let body = Body::try_from(value.get("body").ok_or("body not found")?).map_err(|err| format!("failed to parse body: {}", err))?;

        Ok(Human::new(String::from(name), gender, body, dominant_arm, Inventory::new(), None::<Box<dyn Controller>>))
    }
}
