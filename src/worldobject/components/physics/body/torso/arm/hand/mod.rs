mod worldobject;
mod wieldable;

use serde::ser::SerializeStruct;

use crate::{
    worldobject::components::{
        wielder::{WielderTrait, wieldable::Wieldable},
        container::containable::ContainableTrait,
        physics::PhysicsObjectTrait,
    },
    quantities::{Quantity, mass::Mass, force::Force},
};
use std::{fmt, error::Error as StdError};

pub struct Hand {
    pub base_mass: Quantity<Mass>,
    pub held_item: Option<Wieldable>,
}

pub struct HandDeserializeError;

impl fmt::Display for HandDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandDeserializeError")
    }
}

impl<'de> TryFrom<&serde_json::Value> for Hand {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let base_mass = value.get("base_mass").ok_or(String::from("base_mass not found"))
            .and_then(|v| Quantity::<Mass>::try_from(v.clone()).map_err(|err| format!("failed to parse base_mass: {}", err)))?;
        
        Ok(Hand {
            held_item: None,
            base_mass: base_mass,
        })
    }
}

impl Hand {
    pub fn new(base_mass: Quantity<Mass>, held_item: Option<Wieldable>) -> Hand {
        Hand { base_mass, held_item }
    }
}

impl ContainableTrait for Hand {}

impl WielderTrait for Hand {
    fn wield(&mut self, item: Wieldable) -> Result<(), Box<dyn StdError>> {
        self.held_item = Some(item);
        Ok(())
    }
}

impl PhysicsObjectTrait for Hand {
    fn mass(&self) -> Quantity<Mass> {
        self.base_mass.clone()
    }

    fn apply_force(&self, _force: &Quantity<Force>) -> Result<String, Box<dyn StdError>> {
        Ok(String::from("the hand absorbs the force"))
    }
}

impl serde::Serialize for Hand {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Hand", 1)?;
        state.serialize_field("base_mass", &self.base_mass)?;
        state.end()
    }
}