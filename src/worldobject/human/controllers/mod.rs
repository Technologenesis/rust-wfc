use crate::world;
use crate::worldobject::human::actions;

pub mod terminal;
pub mod network;

pub trait HumanController {
    fn prompt_turn(&mut self, world: &world::World) -> Result<actions::HumanAction, Box<dyn std::error::Error>>;
    fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>>;
}