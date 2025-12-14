pub mod hand;

use crate::quantities::distance;
use crate::quantities::force;
use crate::quantities;
use crate::world;

use crate::components::inventory;
use crate::worldobject::TypedWorldObject;
use crate::worldobject::WorldObject;

use serde::Serialize;
use serde::Deserialize;

use crate::worldobject;

#[derive(Serialize, Deserialize)]
pub struct Arm {
    pub reach: quantities::Quantity<distance::Distance>,
    pub punch_force: quantities::Quantity<force::Force>,

    // mass of the arm without accounting for additional parts
    pub base_mass: quantities::Quantity<quantities::mass::Mass>,

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
    base_mass: quantities::Quantity<quantities::mass::Mass>,
    reach: quantities::Quantity<distance::Distance>,
    punch_force: quantities::Quantity<force::Force>,
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
    type Error = &'static str;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let base_mass = quantities::mass::grams(value.get("base_mass").and_then(|v| v.as_f64()).ok_or("base_mass not found")?);
        let reach = distance::meters(value.get("reach").and_then(|v| v.as_f64()).ok_or("reach not found")?);
        let punch_force = force::newtons(value.get("punch_force").and_then(|v| v.as_f64()).ok_or("punch_force not found")?);

        Ok(Arm { base_mass, reach, punch_force, hand: Some(hand::hand(None::<Box<dyn inventory::InventoryItem>>)) })
    }
}

impl worldobject::TypedWorldObject for Arm {
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
            hand: self.hand.map(
                |hand| <hand::Hand as TypedWorldObject>::dummy(&hand)
            )
        }
    }

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<worldobject::UpdateFn, worldobject::Error> {
        Ok(worldobject::UpdateFn::no_op())
    }

    fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (worldobject::Error, Self)> {
        Ok(*self)
    }

    fn inventory(&self) -> Result<&inventory::Inventory, worldobject::Error> {
        Err(Box::new(ArmInventoryError {}))
    }

    fn inventory_mut(&mut self) -> Result<&mut inventory::Inventory, worldobject::Error> {
        Err(Box::new(ArmInventoryError {}))
    }

    fn mass(&self) -> quantities::Quantity<quantities::mass::Mass> {
        self.hand.as_ref().map(
            |hand| &self.base_mass + &<hand::Hand as WorldObject>::mass(hand)
        ).unwrap_or(self.base_mass.clone())
    }

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, worldobject::Error> {
        Ok(String::from(""))
    }
    
    fn examine(&self) -> String {
        String::from("an arm")
    }

    fn send_message(&mut self, message: String) -> Result<(), worldobject::Error> {
        Ok(())
    }

    fn interact(&mut self) -> Result<String, worldobject::Error> {
        Ok(String::from("you can't think of anything particularly interesting to do with this."))
    }
}

impl inventory::InventoryItem for Arm {
    fn dummy(&self) -> Box<dyn inventory::InventoryItem> {
        Box::new(<Arm as worldobject::TypedWorldObject>::dummy(self))
    }
}