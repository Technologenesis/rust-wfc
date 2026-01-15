use async_trait::async_trait;

use crate::{
    materials::Material,
    quantities::{
        Quantity,
        mass::{
            Mass,
            grams
        },
        force::Force,
        distance::{
            Distance,
            meters
        }
    },
    world::{
        World,
        handle::WorldObjectHandle
    },
    worldobject::{
        Error as WorldObjectError,
        TypedWorldObject,
        fns::update::UpdateFn,
        components::inventory::{
            Inventory,
            item::InventoryItem
        }
    }
};

use std::fmt;
use std::error;

pub struct Sword {
    mass: Quantity<Mass>,
    reach: Quantity<Distance>,
    material: Material
}

impl Sword {
    pub fn new(reach: Quantity<Distance>, material: Material) -> Sword {
        Sword { mass: grams(1500.0), reach, material }
    }
}

#[derive(Debug)]
pub struct SwordInventoryError ();

impl fmt::Display for SwordInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "swords don't have inventories")
    }
}

impl error::Error for SwordInventoryError {}

#[async_trait]
impl TypedWorldObject for Sword {
    type Dummy = Self;
    type CollectInventoryItem = Self;

    fn name(&self) -> String {
        String::from("sword")
    }

    async fn update(&mut self, _: WorldObjectHandle, _: &World) -> Result<UpdateFn, WorldObjectError> {
        Ok(UpdateFn::no_op())
    }

    fn dummy(&self) -> Self {
        Sword { mass: self.mass.clone(), reach: self.reach.clone(), material: self.material.clone() }
    }

    fn examine(&self) -> String {
        format!("a sword forged from {} with a reach of {} meters", self.material, (&self.reach / &meters(1.0)).cancel())
    }

    async fn collect(self: Box<Self>) -> Result<Self, (WorldObjectError, Box<Self>)> {
        Ok(*self)
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        Ok(String::from("nothing happens"))
    }

    fn inventory(&self) -> Result<&Inventory, WorldObjectError> {
        Err(Box::new(SwordInventoryError()))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, WorldObjectError> {
        Err(Box::new(SwordInventoryError()))
    }

    fn mass(&self) -> Quantity<Mass> {
        self.mass.clone()
    }

    async fn apply_force(&mut self, _: &Quantity<Force>) -> Result<String, WorldObjectError> {
        Ok(String::from("the sword bends with the force, but recovers its shape"))
    }

    async fn send_message(&mut self, _: String) -> Result<(), WorldObjectError> {
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
        Box::new(<Sword as TypedWorldObject>::dummy(self))
    }
}
