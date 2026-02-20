use std::ops::DerefMut;

use async_trait::async_trait;

pub mod terminal;
pub mod net;
pub mod commands;

#[async_trait]
pub trait Controller: Send + Sync {
    async fn prompt_turn(&mut self) -> Result<commands::Command, Box<dyn std::error::Error>>;
    async fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl<D: DerefMut + Send + Sync> Controller for D
where D::Target: Controller {
    async fn prompt_turn(&mut self) -> Result<commands::Command, Box<dyn std::error::Error>> {
        self.deref_mut().prompt_turn().await
    }

    async fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        self.deref_mut().display_message(message).await
    }
}