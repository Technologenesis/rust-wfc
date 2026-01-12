use crate::world;
use crate::materials;
use crate::worldobject;
use crate::worldobject::components::inventory::{
    Inventory,
    item::InventoryItem};
use crate::quantities::distance;
use crate::quantities::mass;
use crate::quantities::force;
use crate::quantities;

use std::fmt;
use std::error;

pub struct Sword {
    mass: quantities::Quantity<mass::Mass>,
    reach: quantities::Quantity<distance::Distance>,
    material: materials::Material
}

impl Sword {
    pub fn new(reach: quantities::Quantity<distance::Distance>, material: materials::Material) -> Sword {
        Sword { mass: mass::grams(1500.0), reach, material }
    }
}

#[derive(Debug)]
pub struct SwordCollectError ();

#[derive(Debug)]
pub struct SwordInventoryError ();

impl fmt::Display for SwordInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "swords don't have inventories")
    }
}

impl error::Error for SwordInventoryError {}

impl fmt::Display for SwordCollectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "swords cannot be collected (yet)")
    }
}

impl error::Error for SwordCollectError {}

impl worldobject::TypedWorldObject for Sword {
    type Dummy = Self;
    type CollectInventoryItem = Self;

    fn name(&self) -> String {
        String::from("sword")
    }

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<worldobject::UpdateFn, worldobject::Error> {
        Ok(worldobject::UpdateFn::no_op())
    }

    fn dummy(&self) -> Self {
        Sword { mass: self.mass.clone(), reach: self.reach.clone(), material: self.material.clone() }
    }

    fn examine(&self) -> String {
        format!("a sword forged from {} with a reach of {} meters", self.material, (&self.reach / &distance::meters(1.0)).cancel())
    }

    fn collect(self: Box<Self>) -> Result<Self, (worldobject::Error, Box<Self>)> {
        Ok(*self)
    }

    fn interact(&mut self) -> Result<String, worldobject::Error> {
        Ok(String::from("nothing happens"))
    }

    fn inventory(&self) -> Result<&Inventory, Box<dyn std::error::Error>> {
        Err(Box::new(SwordInventoryError()))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, Box<dyn std::error::Error>> {
        Err(Box::new(SwordInventoryError()))
    }

    fn mass(&self) -> quantities::Quantity<mass::Mass> {
        self.mass.clone()
    }

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, worldobject::Error> {
        Ok(String::from("the sword bends with the force, but recovers its shape"))
    }

    fn send_message(&mut self, message: String) -> Result<(), worldobject::Error> {
        Ok(())
    }

    fn definite_description(&self) -> String {
        String::from("the sword")
    }

    fn pronoun(&self) -> String {
        String::from("it")
    }
}

impl InventoryItem for Sword {
    fn dummy(&self) -> Box<dyn InventoryItem> {
        Box::new(<Sword as worldobject::TypedWorldObject>::dummy(self))
    }
}
