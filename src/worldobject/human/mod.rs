pub mod controllers;
pub mod unsouled;
pub mod actions;

use std::error::Error as StdError;

use futures::future::BoxFuture;
use async_trait::async_trait;

use crate::{
    world::{
        World,
        handle::WorldObjectHandle
    },
    quantities::{
        Quantity,
        force::Force,
        mass::Mass,
        speed::Speed,
        direction::DirectionHorizontal,
        duration::seconds,
        distance::meters
    },
    worldobject::{
        TypedWorldObject,
        Error as WorldObjectError,
        fns::update::UpdateFn,
        components::inventory::{
            Inventory,
            item::none::NoInventoryItem
        }
    }
};

use unsouled::{
    UnsouledHuman,
    body::arm::Arm,
    gender::Gender
};

use actions::{
    HumanAction,
    interact_action::InteractAction
};

pub struct Human {
    unsouled: UnsouledHuman,
    controller: Box<dyn controllers::HumanController>,
}

#[derive(Debug)]
pub struct HumanCollectError ();

impl std::fmt::Display for HumanCollectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "humans can't be collected")
    }
}

#[async_trait]
impl TypedWorldObject for UnsouledHuman {
    type Dummy = Self;
    type CollectInventoryItem = NoInventoryItem; // humans can't be collected

    fn name(&self) -> String {
        self.name.clone()
    }

    fn definite_description(&self) -> String {
        self.name.clone()
    }

    fn pronoun(&self) -> String {
        self.gender.subject_pronoun().to_string()
    }

    async fn collect(self: Box<Self>) -> Result<NoInventoryItem, (WorldObjectError, Box<Self>)> {
        Err((Box::new(HumanCollectError()), self))
    }

    fn dummy(&self) -> Self {
        UnsouledHuman {
            name: self.name.clone(),
            gender: self.gender.clone(),
            mass: self.mass.clone(),
            speed: self.speed.clone(),
            dominant_arm: self.dominant_arm.clone(),
            arm_left: self.arm_left.as_ref().map(
                |arm| <Arm as TypedWorldObject>::dummy(&arm)
            ),
            arm_right: self.arm_right.as_ref().map(
                |arm| <Arm as TypedWorldObject>::dummy(&arm)
            ),
            inventory: self.inventory.dummy()
        }
    }

    fn examine(&self) -> String {
        format!("a human {}; {} name is {}", self.gender.noun(), self.gender.possessive_pronoun(), self.name)
    }

    fn inventory(&self) -> Result<&Inventory, WorldObjectError> {
        Ok(&self.inventory)
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, WorldObjectError> {
        Ok(&mut self.inventory)
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        Ok(format!("{} says \"Hello\".", self.name))
    }

    /*
    fn interact(&self, actor: &world::WorldObjectHandle, world: &mut world::World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    */

    async fn update(&mut self, _: WorldObjectHandle, _: &World) -> Result<UpdateFn, WorldObjectError> {
        Ok(UpdateFn::no_op())
    }

    async fn send_message(&mut self, _: String) -> Result<(), WorldObjectError> {
        Ok(())
    }

    async fn apply_force(&mut self, _: &Quantity<Force>) -> Result<String, WorldObjectError> {
        Ok(format!("{}'s hefty constitution absorbs the force.", self.definite_description()))
    }

    fn mass(&self) -> Quantity<Mass> {
        self.mass.clone()
    }
}

impl TryFrom<&serde_json::Value> for UnsouledHuman {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let name = value.get("name").and_then(|v| v.as_str()).ok_or("name not found")?;
        let gender = value.get("gender").map(|v| Gender::try_from(v)).transpose().map_err(|_| "gender not found")?.ok_or("gender not found")?;

        let speed = Quantity::<Speed>::try_from(value.get("speed").cloned().ok_or("speed not found")?).map_err(|err| format!("failed to parse speed: {}", err))?;
        let arm_left = value.get("arm_left").map(|v| Arm::try_from(v)).transpose().map_err(|err| format!("failed to parse arm_left: {}", err))?;
        let arm_right = value.get("arm_right").map(|v| Arm::try_from(v)).transpose().map_err(|err| format!("failed to parse arm_right: {}", err))?;
        let dominant_arm = value.get("dominant_arm").map(|v| DirectionHorizontal::try_from(v)).transpose().map_err(|err| format!("failed to parse dominant_arm: {}", err))?.ok_or("dominant_arm not found")?;
        let mass = value.get("mass").ok_or(String::from("mass not found"))
            .and_then(|value| Quantity::<Mass>::try_from(value.clone()).map_err(|err| format!("failed to parse mass: {}", err)))?;

        Ok(UnsouledHuman::new(String::from(name), gender, speed, arm_left, arm_right, dominant_arm, mass, Inventory::new()))
    }
}

impl Human {
    pub fn new(
        unsouled: UnsouledHuman,
        controller: impl controllers::HumanController + 'static,
    ) -> Human {
        Human{
            unsouled: unsouled,
            controller: Box::new(controller),
        }
    }

    pub fn desouled(self) -> (UnsouledHuman, Box<dyn controllers::HumanController>) {
        (self.unsouled, self.controller)
    }
}

impl StdError for HumanCollectError {}

#[derive(Debug)]
pub enum HumanUpdateError {
    MoveError(MoveError),
    AttackError(AttackError),
}

impl std::fmt::Display for HumanUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MoveError(move_err) => write!(f, "failed to move: {}", move_err),
            Self::AttackError(attack_err) => write!(f, "failed to attack: {}", attack_err),
        }
    }
}

impl std::error::Error for HumanUpdateError {}

#[derive(Debug)]
pub enum MoveError {
    DistanceTooGreat,
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DistanceTooGreat => write!(f, "distance too great"),
        }
    }
}

impl std::error::Error for MoveError {}

#[derive(Debug)]
pub enum AttackError {
    NoArmProvided,
}

impl std::fmt::Display for AttackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoArmProvided => write!(f, "no arm provided"),
        }
    }
}

impl std::error::Error for AttackError {}

#[async_trait]
impl TypedWorldObject for Human {
    type Dummy = UnsouledHuman;
    // humans can't be collected
    type CollectInventoryItem = NoInventoryItem;

    fn name(&self) -> String {
        self.unsouled.name.clone()
    }

    fn definite_description(&self) -> String {
        self.unsouled.name.clone()
    }

    fn pronoun(&self) -> String {
        self.unsouled.gender.subject_pronoun().to_string()
    }

    async fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (WorldObjectError, Box<Self>)> {
        Err((Box::new(HumanCollectError()), self))
    }

    fn dummy(&self) -> Self::Dummy {
        self.unsouled.dummy()
    }

    fn examine(&self) -> String {
        format!("a human {}; {} name is {}", self.unsouled.gender.noun(), self.unsouled.gender.possessive_pronoun(), self.unsouled.name)
    }

    fn inventory(&self) -> Result<&Inventory, WorldObjectError> {
        Ok(&self.unsouled.inventory)
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, WorldObjectError> {
        Ok(&mut self.unsouled.inventory)
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        Ok(format!("{} says \"Hello\".", self.unsouled.name))
    }

    /*
    fn interact(&self, actor: &world::WorldObjectHandle, world: &mut world::World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    */

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<UpdateFn, WorldObjectError> {
        let action = self.controller.prompt_turn().await?;

        match action {
            HumanAction::Move(actions::move_action::MoveAction {
                direction,
                distance: dist,
            }) => {
                if dist > (seconds(1.0) * self.unsouled.speed.commute()).associate_left().commute().cancel() {
                    return Err(Box::new(HumanUpdateError::MoveError(MoveError::DistanceTooGreat)));
                }

                Ok(UpdateFn(Box::new(move |world: &mut World| -> BoxFuture<Result<Option<String>, WorldObjectError>> {
                    Box::pin(async move {
                        world.move_object(&my_handle, &direction, &dist)
                            .map_err(|err| Box::new(err))?;

                        let dist_f64 = (&dist / &meters(1.0)).cancel().0.0;

                        Ok(Some(format!("you move {} meters {}", dist_f64, direction)))
                    })
                })))
            }
            actions::HumanAction::Examine(actions::examine_action::ExamineAction {
                target_handle
            }) => {
                {
                    let description = world.get_object(&target_handle)
                        .map(|object| object.examine())
                        .map_err(|err| Box::new(err))?;
                    
                    self.controller.display_message(description).await?;
                }
                
                Ok(UpdateFn::no_op())
            },
            actions::HumanAction::Collect(actions::collect_action::CollectAction {
                target_handle
            }) => Ok(UpdateFn(Box::new(move |world: &mut World| {
                    Box::pin(async move {
                        let location = world.locate_object(&target_handle)?;
                        let object = world.take_object(&target_handle)?;

                        let object_desc = object.definite_description();

                        let inventory_item = object.collect().await
                            .or_else(|(err, og_object)| {
                                world.add_object(target_handle.clone(), og_object, location);
                                Err(err)
                        })?;

                        world.give_item_to(&my_handle, inventory_item)
                            .map_err(|err| Box::new(err))?;

                        Ok(Some(format!("you collect {}", object_desc)))
                    })
                })
            )),
            actions::HumanAction::Circumspect => {
                let descs = world.objects.iter()
                    .map(|(handle, (_, object))| format!("- {}: {}", handle, object.examine()))
                    .collect::<Vec<String>>();

                let message = if descs.is_empty() {
                    "You look around and see nothing.".to_string()
                } else {
                    format!("You look around and see the following:\n\t{}", descs.join("\n\t"))
                };

                self.controller.display_message(message).await?;
                
                Ok(UpdateFn::no_op())
            },
            actions::HumanAction::Attack(actions::attack_action::AttackAction {
                left_or_right_arm,
                target_handle
            }) => {
                let arm = match left_or_right_arm.as_ref().unwrap_or(&self.unsouled.dominant_arm) {
                    DirectionHorizontal::Left => &self.unsouled.arm_left,
                    DirectionHorizontal::Right => &self.unsouled.arm_right,
                }.as_ref().ok_or(HumanUpdateError::AttackError(AttackError::NoArmProvided))?;

                let punch_force = arm.punch_force.clone();

                Ok(UpdateFn(Box::new(
                    move |world: &mut World| {
                        Box::pin(async move {
                            let object = world.get_object_mut(&target_handle)
                                .map_err(|err| Box::new(err))?;

                            let object_desc = object.definite_description();

                            let msg = object.apply_force(&punch_force).await
                                .unwrap_or_else(|err| format!("failed to apply force: {}", err));

                            Ok(Some(format!("you punch {}; {}", object_desc, msg)))
                        })
                    }
                )))
            },
            HumanAction::Interact(InteractAction{
                target_handle
            }) => {
                Ok(UpdateFn(Box::new(
                    move |world: &mut World| {
                        Box::pin(async move {
                            let object = world.get_object_mut(&target_handle)
                                .map_err(|err| Box::new(err))?;
            
                            let msg = object.interact().await?;

                            Ok(Some(format!("you pet {}; {}", object.definite_description(), msg)))
                        })
                    }
                )))
            }
            HumanAction::Inventory => {
                let message = {
                    let inventory = &self.unsouled.inventory;

                    let mut descriptions = Vec::new();
                    for handle in inventory.0.keys() {
                        let item = inventory.0.get(handle).unwrap();
                        let item_desc = item.examine();
                        descriptions.push(format!("- {}: {}", String::from(handle.clone()), item_desc));
                    }

                    if descriptions.is_empty() {
                        "You are carrying nothing.".to_string()
                    } else {
                        format!("You are carrying the following:\n\t{}", descriptions.join("\n\t"))
                    }
                };

                self.controller.display_message(message).await?;

                Ok(UpdateFn::no_op())
            },
        }
    }

    async fn send_message(&mut self, message: String) -> Result<(), WorldObjectError> {
        self.controller.display_message(message).await?;
        Ok(())
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, WorldObjectError> {
        self.unsouled.apply_force(force).await
    }

    fn mass(&self) -> Quantity<Mass> {
        self.unsouled.mass.clone()
    }
}
