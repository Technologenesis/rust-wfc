use crate::worldobject::human::{
    controllers,
    actions,
};
use crate::world;

pub struct TerminalHumanController {}

impl controllers::HumanController for TerminalHumanController {
    fn prompt_turn(&mut self, _: &world::World) -> Result<actions::HumanAction, Box<dyn std::error::Error>> {
        println!("Enter your action:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let action = actions::HumanAction::try_from(input.as_str())?;
        Ok(action)
    }

    fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", message);
        Ok(())
    }
}