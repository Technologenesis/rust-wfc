pub mod components;
pub mod linguistics;
pub mod fns;
pub mod human;
pub mod sword;
pub mod wand;
pub mod rat;

use std::error::Error as StdError;

use async_trait::async_trait;

use crate::{
    world::{World, handle::WorldObjectHandle},
    worldobject::fns::update::Action
};

use components::{
    person::Person,
    physics::PhysicsObject,
    container::{
        containable::Containable,
        Container
    },
    controllable::Controllable,
    wielder::{Wielder, wieldable::Wieldable}
};

use linguistics::WorldObjectLinguistics;

#[async_trait]
pub trait WorldObject: Send + Sync {
    // core worldobject methods

    // Update returns an action to be performed by the world on this object's behalf.
    // The action is not executed directly by this object because this would require:
    // - directly borrowing self immutably, and
    // - borrowing the world mutably, which implies a mutable borrow of self
    // This would be a problematic double-borrow.
    async fn update(&self, my_handle: &WorldObjectHandle, world: &World) -> Result<Action, Box<dyn StdError>>;
    // Sends a message to the object.
    async fn send_message(&mut self, message: String) -> Result<(), Box<dyn StdError>>;
    // Returns linguistic information about this object.
    fn linguistics(&self) -> WorldObjectLinguistics;

    // Convenience method: returns the object's name.
    fn name(&self) -> String {
        self.linguistics().name.clone()
    }

    // extention traits
    fn as_controllable(self: Box<Self>) -> Result<Controllable, Box<dyn StdError>>;
    fn as_containable(self: Box<Self>) -> Result<Containable, Box<dyn StdError>>;
    fn as_container(self: Box<Self>) -> Result<Container, Box<dyn StdError>>;
    fn as_person(self: Box<Self>) -> Result<Person, Box<dyn StdError>>;
    fn as_physics_object(self: Box<Self>) -> Result<PhysicsObject, Box<dyn StdError>>;
    fn as_wielder(self: Box<Self>) -> Result<Wielder, Box<dyn StdError>>;
    fn as_wieldable(self: Box<Self>) -> Result<Wieldable, Box<dyn StdError>>;
}