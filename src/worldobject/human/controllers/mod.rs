use crate::world;
use crate::worldobject::human::actions;

use async_trait::async_trait;

pub mod terminal;
pub mod net;

#[async_trait]
pub trait HumanController: Send + Sync {
    async fn prompt_turn(&mut self) -> Result<actions::HumanAction, Box<dyn std::error::Error>>;
    async fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>>;
}