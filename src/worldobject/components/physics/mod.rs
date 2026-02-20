pub mod body;

use std::error::Error as StdError;

use crate::{
    worldobject::WorldObject,
    quantities::{
        Quantity,
        mass::Mass,
        force::Force,
    }
};

use body::Body;

pub type PhysicsObject = Box<dyn PhysicsObjectTrait>;

pub trait PhysicsObjectTrait: WorldObject {
    fn mass(&self) -> Quantity<Mass>;
    fn apply_force(&self, force: &Quantity<Force>) -> Result<String, Box<dyn StdError>>;

    fn as_body(self: Box<Self>) -> Result<Body, Box<dyn StdError>> {
        Err(Box::from(format!("{} is not a body", self.linguistics().name)))
    }
}