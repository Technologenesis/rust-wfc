use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;

use crate::worldobject::human::{
    controllers,
    actions,
};
use crate::world;

pub struct TerminalHumanController {}

#[async_trait]
impl controllers::HumanController for TerminalHumanController {
    async fn prompt_turn(&mut self) -> Result<actions::HumanAction, Box<dyn std::error::Error>> {
        println!("Enter your action:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        let action = actions::HumanAction::try_from(input.as_str())?;
        Ok(action)
    }

    async fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", message);
        Ok(())
    }
}