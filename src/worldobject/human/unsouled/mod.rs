use std::error::Error as StdError;

use serde::{Serialize};
use async_trait::async_trait;

use crate::{
    quantities::{Quantity, mass::Mass, force::Force, direction::DirectionHorizontal},
    world::{
        World,
        handle::WorldObjectHandle
    },
    worldobject::{
        TypedWorldObject,
        Error as WorldObjectError,
        fns::update::Action,
        components::{
            inventory::{Inventory, item::none::NoInventoryItem},
            gender::Gender,
            body::Body
        }
    }
};

#[derive(Debug)]
pub struct HumanCollectError ();

impl std::fmt::Display for HumanCollectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "humans can't be collected")
    }
}

impl StdError for HumanCollectError {}

// An UnsouledHuman is just a human character without a controller.
// The `dummy` method of human returns an UnsouledHuman.
#[derive(Serialize)]
pub struct UnsouledHuman {
    // identity
    pub name: String,
    pub gender: Gender,

    // body
    pub body: Body,

    // state
    pub inventory: Inventory,

    // misc
    pub dominant_arm: DirectionHorizontal,
}

impl UnsouledHuman {
    pub fn new(
        name: String,
        gender: Gender,
        body: Body,
        dominant_arm: DirectionHorizontal,
        inventory: Inventory,
    ) -> UnsouledHuman {
        UnsouledHuman {
            name,
            gender,
            body,
            dominant_arm,
            inventory,
        }
    }
}

#[async_trait]
impl TypedWorldObject for UnsouledHuman {
    type Dummy = Self;
    type CollectInventoryItem = NoInventoryItem; // humans can't be collected

    fn name(&self) -> String {
        self.name.clone()
    }

    fn indefinite_description(&self) -> String {
        format!("a {}", self.gender.noun())
    }

    fn definite_description(&self) -> String {
        self.name.clone()
    }

    fn pronoun(&self) -> String {
        self.gender.subject_pronoun().to_string()
    }

    async fn collect(self: Box<Self>) -> Result<NoInventoryItem, (WorldObjectError, Box<Self>)> {
        Err((Box::new(HumanCollectError()), self))
    }

    fn dummy(&self) -> Self {
        UnsouledHuman {
            dominant_arm: self.dominant_arm.clone(),
            name: self.name.clone(),
            gender: self.gender.clone(),
            body: self.body.dummy(),
            inventory: self.inventory.dummy()
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

    /*
    fn interact(&self, actor: &world::WorldObjectHandle, world: &mut world::World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    */

    async fn update(&mut self, _: WorldObjectHandle, _: &World) -> Result<Action, WorldObjectError> {
        Ok(Action::no_op())
    }

    async fn send_message(&mut self, _: String) -> Result<(), WorldObjectError> {
        Ok(())
    }

    async fn apply_force(&mut self, _: &Quantity<Force>) -> Result<String, WorldObjectError> {
        Ok(format!("{}'s hefty constitution absorbs the force.", self.definite_description()))
    }

    fn mass(&self) -> Quantity<Mass> {
        self.body.mass()
    }
}

impl TryFrom<&serde_json::Value> for UnsouledHuman {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let name = value.get("name").and_then(|v| v.as_str()).ok_or("name not found")?;
        let gender = value.get("gender").map(|v| Gender::try_from(v)).transpose().map_err(|_| "gender not found")?.ok_or("gender not found")?;

        let dominant_arm = value.get("dominant_arm").map(|v| DirectionHorizontal::try_from(v)).transpose().map_err(|err| format!("failed to parse dominant_arm: {}", err))?.ok_or("dominant_arm not found")?;

        let body = Body::try_from(value.get("body").ok_or("body not found")?).map_err(|err| format!("failed to parse body: {}", err))?;

        Ok(UnsouledHuman::new(String::from(name), gender, body, dominant_arm, Inventory::new()))
    }
}
