use crate::{
    world::{World, handle::WorldObjectHandle},
    worldobject::{
        components::inventory::item::InventoryItem,
        fns::update::Action
    }
};
use super::Hand;

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
        Box::new(self.dummy())
    }

    fn use_item(&mut self, _: &World, _: WorldObjectHandle, _: Option<WorldObjectHandle>) -> Result<Action, Box<dyn std::error::Error>> {
        return Err(Box::new(HandUseError));
    }
}