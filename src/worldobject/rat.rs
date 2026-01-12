use crate::worldobject;
use crate::quantities;
use crate::quantities::mass;
use crate::quantities::speed;
use crate::quantities::force;
use crate::worldobject::components::inventory::{
    Inventory,
    item::{
        InventoryItem,
        none::NoInventoryItem
    }
};
use crate::world;

pub struct Rat {
    mass: quantities::Quantity<mass::Mass>,
    speed: quantities::Quantity<speed::Speed>,
    alive: bool,
}

#[derive(Debug)]
pub struct RatInventoryError ();

impl std::fmt::Display for RatInventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rats don't have inventories")
    }
}

impl Rat {
    pub fn new(mass: quantities::Quantity<mass::Mass>, speed: quantities::Quantity<speed::Speed>) -> Rat {
        Rat { mass, speed, alive: true }
    }
}

impl std::error::Error for RatInventoryError {}

impl InventoryItem for Rat {
    fn dummy(&self) -> Box<dyn InventoryItem> {
        Box::new(<Rat as worldobject::TypedWorldObject>::dummy(self))
    }
}

impl worldobject::TypedWorldObject for Rat {
    type Dummy = Self;
    type CollectInventoryItem = Self;

    fn name(&self) -> String {
        String::from("rat")
    }

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<worldobject::UpdateFn, worldobject::Error> {
        Ok(worldobject::UpdateFn::no_op())
    }

    fn examine(&self) -> String {
        if self.alive {
            String::from("a rat")
        } else {
            String::from("a dead rat")
        }
    }

    fn dummy(&self) -> Self {
        Rat::new(self.mass.clone(), self.speed.clone())
    }

    fn definite_description(&self) -> String {
        if self.alive {
            String::from("the rat")
        } else {
            String::from("the dead rat")
        }
    }

    fn pronoun(&self) -> String {
        String::from("it")
    }

    fn collect(self: Box<Self>) -> Result<Self, (worldobject::Error, Box<Self>)> {
        Ok(*self)
    }

    fn interact(&mut self) -> Result<String, worldobject::Error> {
        Ok(String::from("the rat squeaks happily."))
    }

    fn inventory(&self) -> Result<&Inventory, worldobject::Error> {
        Err(Box::new(RatInventoryError()))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, worldobject::Error> {
        Err(Box::new(RatInventoryError()))
    }

    fn mass(&self) -> quantities::Quantity<mass::Mass> {
        self.mass.clone()
    }

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, worldobject::Error> {
        if force > &force::newtons(100.0) {
            self.alive = false;
            Ok(String::from("the rat is crushed by the force"))
        } else {
            Ok(String::from("the rat survives the force"))
        }
    }

    fn send_message(&mut self, message: String) -> Result<(), worldobject::Error> {
        Ok(())
    }
}