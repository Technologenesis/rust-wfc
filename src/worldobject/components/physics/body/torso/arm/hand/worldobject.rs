use async_trait::async_trait;

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    worldobject::{
        linguistics::WorldObjectLinguistics,
        WorldObject,
        fns::update::Action,
        components::{
            container::{
                Container,
                containable::Containable
            },
            person::Person,
            physics::PhysicsObject,
            wielder::{Wielder, wieldable::Wieldable},
            controllable::Controllable
        }
    },
    world::{
        handle::WorldObjectHandle,
        World
    }
};

use super::Hand;

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
    // core worldobject methods
    async fn update(&self, my_handle: &WorldObjectHandle, world: &World) -> Result<Action, Box<dyn StdError>> {
        Ok(Action::no_op())
    }

    fn linguistics(&self) -> WorldObjectLinguistics {
        WorldObjectLinguistics {
            name: String::from("hand"),
            definite_description: String::from("the hand"),
            indefinite_description: String::from("a hand"),
            pronoun: String::from("it"),
        }
    }

    // Sends a message to the object.
    fn send_message(&mut self, message: String) -> Result<(), Box<dyn StdError>> {
        Ok(())
    }

    // extention traits
    fn as_controllable(self: Box<Self>) -> Result<Box<Controllable>, Box<dyn StdError>> {
        Err(Box::from(format!("{} cannot be ensouled", self.linguistics().name)))
    }
    fn as_containable(self: Box<Self>) -> Result<Containable, Box<dyn StdError>> {
        Ok(self)
    }
    fn as_container(self: Box<Self>) -> Result<Container, Box<dyn StdError>> {
        Err(Box::from(format!("{} cannot contain items", self.linguistics().name)))
    }
    fn as_person(self: Box<Self>) -> Result<Box<Person>, Box<dyn StdError>> {
        Err(Box::from(format!("{} is not a person", self.linguistics().name)))
    }
    fn as_physics_object(self: Box<Self>) -> Result<PhysicsObject, Box<dyn StdError>> {
        Ok(self)
    }
    fn as_wielder(self: Box<Self>) -> Result<Box<Wielder>, Box<dyn StdError>> {
        Ok(self)
    }
    fn as_wieldable(self: Box<Self>) -> Result<Box<Wieldable>, Box<dyn StdError>> {
        Ok(self)
    }
}
