use std::error::Error as StdError;

use async_trait::async_trait;

use crate::{
    quantities::{
        Quantity,
        mass::Mass,
        force::Force,
        speed::Speed,
    },
    world::{World, handle::WorldObjectHandle},
    worldobject::{
        WorldObject,
        linguistics::WorldObjectLinguistics,
        fns::update::Action,
        components::{
            container::{Container, containable::Containable},
            controllable::Controllable,
            person::Person,
            physics::PhysicsObject,
            wielder::{Wielder, wieldable::Wieldable},
        },
    },
};

pub struct Rat {
    pub mass: Quantity<Mass>,
    pub speed: Quantity<Speed>,
}

impl Rat {
    pub fn new(mass: Quantity<Mass>, speed: Quantity<Speed>) -> Rat {
        Rat { mass, speed }
    }
}

#[async_trait]
impl WorldObject for Rat {
    async fn update(&self, _my_handle: &WorldObjectHandle, _world: &World) -> Result<Action, Box<dyn StdError>> {
        Ok(Action::no_op())
    }

    fn linguistics(&self) -> WorldObjectLinguistics {
        WorldObjectLinguistics {
            name: String::from("rat"),
            definite_description: String::from("the rat"),
            indefinite_description: String::from("a rat"),
            pronoun: String::from("it"),
        }
    }

    async fn send_message(&mut self, _message: String) -> Result<(), Box<dyn StdError>> {
        Ok(())
    }

    fn as_controllable(self: Box<Self>) -> Result<Controllable, Box<dyn StdError>> {
        Err(Box::from("rat is not controllable"))
    }

    fn as_containable(self: Box<Self>) -> Result<Containable, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_container(self: Box<Self>) -> Result<Container, Box<dyn StdError>> {
        Err(Box::from("rat is not a container"))
    }

    fn as_person(self: Box<Self>) -> Result<Person, Box<dyn StdError>> {
        Err(Box::from("rat is not a person"))
    }

    fn as_physics_object(self: Box<Self>) -> Result<PhysicsObject, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_wielder(self: Box<Self>) -> Result<Wielder, Box<dyn StdError>> {
        Err(Box::from("rat is not a wielder"))
    }

    fn as_wieldable(self: Box<Self>) -> Result<Wieldable, Box<dyn StdError>> {
        Ok(self)
    }
}

impl crate::worldobject::components::physics::PhysicsObjectTrait for Rat {
    fn mass(&self) -> Quantity<Mass> {
        self.mass.clone()
    }

    fn apply_force(&self, _force: &Quantity<Force>) -> Result<String, Box<dyn StdError>> {
        Ok(String::from("the rat squeaks"))
    }
}

impl crate::worldobject::components::container::containable::ContainableTrait for Rat {}

impl crate::worldobject::components::wielder::wieldable::WieldableTrait for Rat {
    fn wieldable_name(&self) -> String {
        String::from("rat")
    }
}
