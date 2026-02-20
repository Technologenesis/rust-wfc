pub mod actions;
pub mod controllable;

use std::error::Error as StdError;
use async_trait::async_trait;
use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::{
    quantities::direction::DirectionHorizontal, world::{
        World,
        handle::WorldObjectHandle
    }, worldobject::{
        linguistics::WorldObjectLinguistics,
        WorldObject, components::{
            controllable::{Controllable, controller::Controller},
            person::{Person, gender::Gender},
            physics::{PhysicsObject, body::Body},
            container::{
                Container,
                containable::Containable
            },
            wielder::{
                Wielder,
                wieldable::Wieldable
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
    pub inventory: Vec<Containable>
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
        controller: Option<Controller>,
    ) -> Human {
        Human{
            name,
            gender,
            body,
            dominant_arm,
            inventory: Vec::new()
        }
    }
}

#[async_trait]
impl WorldObject for Human {
    // core worldobject methods
    async fn update(&self, my_handle: &WorldObjectHandle, world: &World) -> Result<Action, Box<dyn StdError>> {
        Ok(Action::no_op())
    }

    // Returns linguistic information about this object.
    fn linguistics(&self) -> WorldObjectLinguistics {
        WorldObjectLinguistics {
            name: self.name.clone(),
            definite_description: self.definite_description.clone(),
            indefinite_description: self.indefinite_description.clone(),
            pronoun: self.pronoun.clone(),
        }
    }

    // Sends a message to the object.
    async fn send_message(&mut self, message: String) -> Result<(), Box<dyn StdError>> {
        match &mut self.controller {
            Some(controller) => controller.display_message(message).await,
            None => Ok(()),
        }
    }

    // extension traits
    fn as_controllable(self: Box<Self>) -> Result<Box<Controllable>, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_containable(self: Box<Self>) -> Result<Containable, Box<dyn StdError>> {
        Err(Box::from(format!("{} is not a containable", self.linguistics().name)))
    }

    fn as_container(self: Box<Self>) -> Result<Container, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_person(self: Box<Self>) -> Result<Box<Person>, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_physics_object(self: Box<Self>) -> Result<PhysicsObject, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_wielder(self: Box<Self>) -> Result<Box<Wielder>, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_wieldable(self: Box<Self>) -> Result<Box<Wieldable>, Box<dyn StdError>> {
        Err(Box::from(format!("{} is not a wieldable", self.linguistics().name)))
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

        Ok(Human::new(String::from(name), gender, body, dominant_arm, None::<Controller>))
    }
}