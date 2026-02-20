pub mod hand;

use serde::{
    Serialize,
    Deserialize
};

use std::error::Error as StdError;

use async_trait::async_trait;

use crate::{
    quantities::{
        self,
        Quantity,
        distance::{Distance, meters},
        force::Force,
        mass::Mass
    },
    world::{
        World,
        handle::WorldObjectHandle,
    },
    worldobject::{
        WorldObject,
        linguistics::WorldObjectLinguistics,
        fns::update::Action,
        components::{
            container::{
                Container,
                containable::{Containable, ContainableTrait}
            },
            person::Person,
            physics::{PhysicsObject, PhysicsObjectTrait},
            wielder::{Wielder, WielderTrait, wieldable::{Wieldable, WieldableTrait}},
            controllable::Controllable
        }
    }
};

use hand::Hand;

#[derive(Serialize)]
pub struct Arm {
    pub reach: Quantity<Distance>,
    pub punch_force: Quantity<Force>,

    // mass of the arm without accounting for additional parts
    pub base_mass: Quantity<Mass>,

    pub hand: Option<hand::Hand>,
}

#[derive(Debug)]
pub struct ArmInventoryError;

impl std::error::Error for ArmInventoryError {}

impl std::fmt::Display for ArmInventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "arms don't have inventories")
    }
}

#[derive(Debug)]
pub struct ArmControllerError;

impl std::fmt::Display for ArmControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArmControllerError")
    }
}

impl std::error::Error for ArmControllerError {}

pub fn arm(
    base_mass: Quantity<Mass>,
    reach: Quantity<Distance>,
    punch_force: Quantity<Force>,
    hand: Option<hand::Hand>
) -> Arm {
    Arm {
        base_mass,
        reach,
        punch_force,
        hand,
    }
}

impl Arm {
    pub fn new(
        base_mass: Quantity<Mass>,
        reach: Quantity<Distance>,
        punch_force: Quantity<Force>,
        hand: Option<hand::Hand>
    ) -> Arm {
        arm(base_mass, reach, punch_force, hand)
    }

    pub fn dummy(&self) -> Arm {
        Arm {
            base_mass: self.base_mass.clone(),
            reach: self.reach.clone(),
            punch_force: self.punch_force.clone(),
            hand: self.hand.as_ref().map(|h| hand::Hand::new(h.base_mass.clone(), None)),
        }
    }
}

impl TryFrom<&serde_json::Value> for Arm {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let base_mass = value.get("base_mass").ok_or(String::from("base_mass not found"))
            .and_then(|value| Quantity::<Mass>::try_from(value.clone()).map_err(|err| format!("failed to parse base_mass: {}", err)))?;
        let hand = value.get("hand").map(|v| hand::Hand::try_from(v)).transpose().map_err(|err| format!("failed to parse hand: {}", err))?;
        let reach = meters(value.get("reach").and_then(|v| v.as_f64()).ok_or("reach not found")?);
        let punch_force = value.get("punch_force").ok_or(String::from("punch_force not found"))
            .and_then(|value| Quantity::<Force>::try_from(value.clone()).map_err(|err| format!("failed to parse punch_force: {}", err)))?;

        Ok(Arm { base_mass, reach, punch_force, hand })
    }
}

#[async_trait]
impl WorldObject for Arm {
    async fn update(&self, my_handle: &WorldObjectHandle, world: &World) -> Result<Action, Box<dyn StdError>> {
        Ok(Action::no_op())
    }

    fn linguistics(&self) -> WorldObjectLinguistics {
        WorldObjectLinguistics {
            name: String::from("arm"),
            definite_description: String::from("the arm"),
            indefinite_description: String::from("an arm"),
            pronoun: String::from("it"),
        }
    }
    
    async fn send_message(&mut self, _message: String) -> Result<(), Box<dyn StdError>> {
        Ok(())
    }

    // extention traits
    fn as_controllable(self: Box<Self>) -> Result<Controllable, Box<dyn StdError>> {
        Err(Box::from(format!("{} cannot be ensouled", self.linguistics().name)))
    }
    fn as_containable(self: Box<Self>) -> Result<Containable, Box<dyn StdError>> {
        Ok(self)
    }
    fn as_container(self: Box<Self>) -> Result<Container, Box<dyn StdError>> {
        Err(Box::from(format!("{} cannot contain items", self.linguistics().name)))
    }
    fn as_person(self: Box<Self>) -> Result<Person, Box<dyn StdError>> {
        Err(Box::from(format!("{} is not a person", self.linguistics().name)))
    }
    fn as_physics_object(self: Box<Self>) -> Result<PhysicsObject, Box<dyn StdError>> {
        Ok(self)
    }
    fn as_wielder(self: Box<Self>) -> Result<Wielder, Box<dyn StdError>> {
        Ok(self)
    }
    fn as_wieldable(self: Box<Self>) -> Result<Wieldable, Box<dyn StdError>> {
        Ok(self)
    }
}

impl ContainableTrait for Arm {}

impl PhysicsObjectTrait for Arm {
    fn mass(&self) -> Quantity<Mass> {
        self.base_mass.clone()
    }

    fn apply_force(&self, _force: &Quantity<Force>) -> Result<String, Box<dyn StdError>> {
        Ok(String::from("the arm absorbs the force"))
    }
}

impl WielderTrait for Arm {
    fn wield(&mut self, item: Wieldable) -> Result<(), Box<dyn StdError>> {
        let hand = self.hand.as_mut().ok_or(ArmWieldError::NoHand)?;
        hand.held_item = Some(item);
        Ok(())
    }
}

impl WieldableTrait for Arm {}

#[derive(Debug)]
pub struct ArmUseError;

impl std::error::Error for ArmUseError {}

impl std::fmt::Display for ArmUseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "you can't think of anything particularly interesting to do with this arm")
    }
}

#[derive(Debug)]
pub enum ArmWieldError {
    NoHand,
}

impl std::error::Error for ArmWieldError {}

impl std::fmt::Display for ArmWieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoHand => write!(f, "arm has no hand"),
        }
    }
}