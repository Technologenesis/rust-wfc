use async_trait::async_trait;

pub mod terminal;
pub mod net;
pub mod commands;

#[async_trait]
pub trait Controller: Send + Sync {
    async fn prompt_turn(&mut self) -> Result<commands::Command, Box<dyn std::error::Error>>;
    async fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>>;
}