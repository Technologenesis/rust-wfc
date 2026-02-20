use crate::worldobject::{
    WorldObject,    
    components::controllable::{Controllable, controller::Controller}
};

pub struct ControlledWorldObject {
    world_object: Controllable,
    controller: Controller,
}

impl WorldObject for ControlledWorldObject {
    fn update(&self, my_handle: &WorldObjectHandle, world: &World) -> Result<Action, Box<dyn StdError>> {
        let involuntary_action = self.world_object.update(my_handle, world).await?;

        let command = self.controller.prompt_turn().await?;
        match command {
            Command::Interact(interact_command) => {
                self.world_object.interact(interact_command).await?;
            }
            Command::Collect(collect_command) => {
                self.world_object.collect(collect_command).await?;
            }
            Command::Attack(attack_command) => {
                self.world_object.attack(attack_command).await?;
            }
            Command::Examine(examine_command) => {
                self.world_object.examine(examine_command).await?;
            }
            Command::Wield(wield_command) => {
                self.world_object.wield(wield_command).await?;
            }
            Command::Circumspect => {
                self.world_object.circumspect().await?;
            }
            Command::Inventory => {
                self.world_object.inventory().await?;
            }
            Command::Use(use_command) => {
                self.world_object.use_wielded_item(use_command).await?;
            }
            _ => {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid command")));
            }
        }

        Ok(Action::None)
    }
}