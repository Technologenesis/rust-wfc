use async_trait::async_trait;

use crate::worldobject::components::controllers::commands::Command;

use super::Controller;

pub struct TerminalHumanController {}

#[async_trait]
impl Controller for TerminalHumanController {
    async fn prompt_turn(&mut self) -> Result<Command, Box<dyn std::error::Error>> {
        println!("Enter your action:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        let action = Command::try_from(input.as_str())?;
        Ok(action)
    }

    async fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", message);
        Ok(())
    }
}