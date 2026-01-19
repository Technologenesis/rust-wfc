use async_trait::async_trait;

use crate::{
    world::{
        World,
        handle::WorldObjectHandle
    },
    worldobject::{
        TypedWorldObject,
        Error as WorldObjectError,
        fns::update::Action,
        components::inventory::{
            Inventory,
            item::{
                InventoryItem,
            }
        }
    },
    quantities::{
        Quantity,
        mass::Mass,
        speed::Speed,
        force::{
            Force,
            newtons
        }
    }
};

pub struct Rat {
    mass: Quantity<Mass>,
    speed: Quantity<Speed>,
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
    pub fn new(mass: Quantity<Mass>, speed: Quantity<Speed>) -> Rat {
        Rat { mass, speed, alive: true }
    }
}

impl std::error::Error for RatInventoryError {}

#[derive(Debug)]
pub struct RatUseError;

impl std::error::Error for RatUseError {}

impl std::fmt::Display for RatUseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "you can't think of anything particularly interesting to do with this rat")
    }
}

impl InventoryItem for Rat {
    fn dummy(&self) -> Box<dyn InventoryItem> {
        Box::new(<Rat as TypedWorldObject>::dummy(self))
    }

    fn use_item(&mut self, _: &World, _: Option<WorldObjectHandle>) -> Result<Action, Box<dyn std::error::Error>> {
        return Err(Box::new(RatUseError));
    }
}

#[async_trait]
impl TypedWorldObject for Rat {
    type Dummy = Self;
    type CollectInventoryItem = Self;

    fn name(&self) -> String {
        String::from("rat")
    }

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, WorldObjectError> {
        Ok(Action::no_op())
    }

    fn examine(&self) -> String {
        if self.alive {
            String::from("a rat")
        } else {
            String::from("a dead rat")
        }
    }

    fn dummy(&self) -> Self {
        Rat{
            mass: self.mass.clone(),
            speed: self.speed.clone(),
            alive: self.alive
        }
    }

    fn definite_description(&self) -> String {
        if self.alive {
            String::from("the rat")
        } else {
            String::from("the dead rat")
        }
    }

    fn indefinite_description(&self) -> String {
        if self.alive {
            String::from("a rat")
        } else {
            String::from("a dead rat")
        }
    }

    fn pronoun(&self) -> String {
        String::from("it")
    }

    async fn collect(self: Box<Self>) -> Result<Self, (WorldObjectError, Box<Self>)> {
        Ok(*self)
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        if self.alive {
            Ok(String::from("the rat squeaks happily."))
        } else {
            Ok(String::from("the rat does nothing."))
        }
    }

    fn inventory(&self) -> Result<&Inventory, WorldObjectError> {
        Err(Box::new(RatInventoryError()))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, WorldObjectError> {
        Err(Box::new(RatInventoryError()))
    }

    fn mass(&self) -> Quantity<Mass> {
        self.mass.clone()
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, WorldObjectError> {
        if force > &newtons(100.0) {
            self.alive = false;
            Ok(String::from("the rat is crushed by the force"))
        } else {
            Ok(String::from("the rat survives the force"))
        }
    }

    async fn send_message(&mut self, message: String) -> Result<(), WorldObjectError> {
        Ok(())
    }
}