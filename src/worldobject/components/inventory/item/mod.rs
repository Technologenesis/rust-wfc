pub mod none;
pub mod sword;
pub mod wand;

use async_trait::async_trait;
use serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    world::{
        World,
        handle::WorldObjectHandle,
    },
    worldobject::{
        WorldObject,
        fns::update::Action,
        Error as WorldObjectError
    },
    quantities::{
        Quantity,
        mass::Mass,
        force::Force
    }
};

use super::Inventory;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct InventoryItemHandle(uuid::Uuid);

impl TryFrom<&str> for InventoryItemHandle {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(InventoryItemHandle(Uuid::try_parse(value).map_err(|_| "invalid UUID")?))
    }
}

impl std::fmt::Display for InventoryItemHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl InventoryItemHandle {
    pub fn new() -> InventoryItemHandle {
        InventoryItemHandle(uuid::Uuid::new_v4())
    }
}

impl From<InventoryItemHandle> for String {
    fn from(handle: InventoryItemHandle) -> String {
        handle.0.to_string()
    }
}

pub trait InventoryItem: WorldObject {
    fn dummy(&self) -> Box<dyn InventoryItem>;
    fn use_item(&mut self, world: &World, target_handle: Option<WorldObjectHandle>) -> Result<Action, Box<dyn std::error::Error>>;
}

#[async_trait]
impl WorldObject for Box<dyn InventoryItem> {
    fn name(&self) -> String {
        (**self).name()
    }

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &crate::world::World) -> Result<Action, WorldObjectError> {
        (**self).update(my_handle, world).await
    }

    fn examine(&self) -> String {
        (**self).examine()
    }

    fn dummy(&self) -> Box<dyn WorldObject> {
        WorldObject::dummy(&**self)
    }

    fn definite_description(&self) -> String {
        (**self).definite_description()
    }

    fn indefinite_description(&self) -> String {
        (**self).indefinite_description()
    }

    fn pronoun(&self) -> String {
        (**self).pronoun()
    }

    async fn collect(self: Box<Self>) -> Result<Box<dyn InventoryItem>, (WorldObjectError, Box<dyn WorldObject>)> {
        (*self).collect().await
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        (**self).interact().await
    }

    fn inventory(&self) -> Result<&Inventory, Box<dyn std::error::Error>> {
        (**self).inventory()
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, Box<dyn std::error::Error>> {
        (**self).inventory_mut()
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, Box<dyn std::error::Error>> {
        (**self).apply_force(force).await
    }

    fn mass(&self) -> Quantity<Mass> {
        (**self).mass()
    }

    async fn send_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        (**self).send_message(message).await
    }
}

#[async_trait]
impl InventoryItem for Box<dyn InventoryItem> {
    fn dummy(&self) -> Box<dyn InventoryItem> {
        InventoryItem::dummy(&(**self))
    }

    fn use_item(&mut self, world: &World, target_handle: Option<WorldObjectHandle>) -> Result<Action, Box<dyn std::error::Error>> {
        (**self).use_item(world, target_handle)
    }
}