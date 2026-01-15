pub mod hand;

use serde::{
    Serialize,
    Deserialize
};

use async_trait::async_trait;

use crate::{
    quantities::{
        self, Quantity, distance::{
            Distance,
            meters
        }, force::{
            Force,
            newtons
        }, mass::{
            Mass,
            grams
        }
    },
    world::{
        World,
        handle::WorldObjectHandle,
    },
    worldobject::{
        Error, TypedWorldObject, UpdateFn, WorldObject, components::inventory::{
            Inventory,
            item::InventoryItem
        }
    }
};

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
impl TypedWorldObject for Arm {
    type Dummy = Self;
    type CollectInventoryItem = Self;

    fn name(&self) -> String {
        String::from("arm")
    }

    fn definite_description(&self) -> String {
        String::from("an arm")
    }

    fn pronoun(&self) -> String {
        String::from("it")
    }

    fn dummy(&self) -> Self::Dummy {
        Arm {
            base_mass: self.base_mass.clone(),
            reach: self.reach.clone(),
            punch_force: self.punch_force.clone(),
            hand: self.hand.as_ref().map(
                |hand| <hand::Hand as TypedWorldObject>::dummy(&hand)
            )
        }
    }

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<UpdateFn, Error> {
        Ok(UpdateFn::no_op())
    }

    async fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (Error, Box<Self>)> {
        Ok(*self)
    }

    fn inventory(&self) -> Result<&Inventory, Error> {
        Err(Box::new(ArmInventoryError {}))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, Error> {
        Err(Box::new(ArmInventoryError {}))
    }

    fn mass(&self) -> Quantity<Mass> {
        self.hand.as_ref().map(
            |hand| &self.base_mass + &<hand::Hand as WorldObject>::mass(hand)
        ).unwrap_or(self.base_mass.clone())
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, Error> {
        Ok(String::from(""))
    }
    
    fn examine(&self) -> String {
        String::from("an arm")
    }

    async fn send_message(&mut self, message: String) -> Result<(), Error> {
        Ok(())
    }

    async fn interact(&mut self) -> Result<String, Error> {
        Ok(String::from("you can't think of anything particularly interesting to do with this."))
    }
}

impl InventoryItem for Arm {
    fn dummy(&self) -> Box<dyn InventoryItem> {
        Box::new(<Arm as TypedWorldObject>::dummy(self))
    }
}