use serde::ser::SerializeStruct;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::{
    worldobject::components::inventory::{
        Inventory,
        item::InventoryItem
    },
    quantities::{
        Quantity,
        mass::Mass,
        force::Force
    },
    worldobject::{
        TypedWorldObject,
        Error as WorldObjectError,
        fns::update::Action
    },
    world::{
        handle::WorldObjectHandle,
        World
    },
    worldobject::components::controllers::Controller
};
use std::fmt;

pub struct Hand {
    base_mass: Quantity<Mass>,
    held_item: Option<Box<dyn InventoryItem>>,
}

#[derive(Debug)]
pub struct HandInventoryError;

impl fmt::Display for HandInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandInventoryError")
    }
}

impl std::error::Error for HandInventoryError {}

pub struct HandDeserializeError;

impl fmt::Display for HandDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandDeserializeError")
    }
}

#[derive(Debug)]
pub struct HandControllerError;

impl std::fmt::Display for HandControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandControllerError")
    }
}

impl std::error::Error for HandControllerError {}

#[async_trait]
impl TypedWorldObject for Hand {
    type Dummy = Self;
    type CollectInventoryItem = Self;

    fn name(&self) -> String {
        String::from("hand")
    }

    fn definite_description(&self) -> String {
        String::from("the hand")
    }

    fn indefinite_description(&self) -> String {
        String::from("a hand")
    }

    fn pronoun(&self) -> String {
        String::from("it")
    }

    fn dummy(&self) -> Self::Dummy {
        Hand {
            held_item: self.held_item.as_ref().map(
                |held_item| InventoryItem::dummy(&*held_item)
            ),
            base_mass: self.base_mass.clone()
        }
    }

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, WorldObjectError> {
        Ok(Action::no_op())
    }

    async fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (WorldObjectError, Box<Self>)> {
        Ok(*self)
    }

    fn inventory(&self) -> Result<&Inventory, WorldObjectError> {
        Err(Box::new(HandInventoryError {}))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, WorldObjectError> {
        Err(Box::new(HandInventoryError {}))
    }

    fn mass(&self) -> Quantity<Mass> {
        let mut ret = self.base_mass.clone();

        if let Some(held_item) = &self.held_item {
            ret = ret + held_item.mass();
        }

        ret
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, WorldObjectError> {
        Ok(String::from(""))
    }
    
    fn examine(&self) -> String {
        String::from("an arm")
    }

    async fn send_message(&mut self, message: String) -> Result<(), WorldObjectError> {
        Ok(())
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        Ok(String::from("you can't think of anything particularly interesting to do with this."))
    }

    fn controller(&self) -> Result<&dyn Controller, WorldObjectError> {
        Err(Box::new(HandControllerError))
    }

    fn controller_mut(&mut self) -> Result<&mut dyn Controller, WorldObjectError> {
        Err(Box::new(HandControllerError))
    }
    
    fn take_controller(&mut self) -> Result<Box<dyn Controller>, WorldObjectError> {
        Err(Box::new(HandControllerError))
    }

    fn set_controller<C: Controller + 'static>(&mut self, controller: C) -> Result<(), (C, WorldObjectError)> {
        Err((controller, Box::new(HandControllerError)))
    }
}

#[derive(Debug)]
pub struct HandUseError;

impl std::error::Error for HandUseError {}

impl std::fmt::Display for HandUseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "you can't think of anything particularly interesting to do with this hand")
    }
}

impl InventoryItem for Hand {
    fn dummy(&self) -> Box<dyn InventoryItem> {
        Box::new(<Hand as TypedWorldObject>::dummy(self))
    }

    fn use_item(&mut self, _: &World, _: WorldObjectHandle, _: Option<WorldObjectHandle>) -> Result<Action, Box<dyn std::error::Error>> {
        return Err(Box::new(HandUseError));
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

impl serde::Serialize for Hand {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Hand", 1)?;
        state.serialize_field("base_mass", &self.base_mass)?;
        state.end()
    }
}

pub fn hand(base_mass: Quantity<Mass>, held_item: Option<impl InventoryItem + 'static>) -> Hand {
    Hand {
        held_item: held_item.map(|item| {
            let item: Box<dyn InventoryItem> = Box::new(item);
            item
        }),
        base_mass,
    }
}

impl Hand {
    pub fn wield(&mut self, item: Box<dyn InventoryItem>) {
        self.held_item = Some(item);
    }

    pub fn wielded_item(&self) -> Option<&dyn InventoryItem> {
        self.held_item.as_ref().map(|item| item.as_ref())
    }

    pub fn wielded_item_mut(&mut self) -> Option<&mut dyn InventoryItem> {
        self.held_item.as_mut().map(|item| {
            item.as_mut()
        })
    }
}