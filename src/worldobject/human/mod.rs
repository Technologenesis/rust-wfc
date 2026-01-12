pub mod actions;
pub mod controllers;
pub mod body;
pub mod gender;

use serde;
use serde::de::Error;
use serde::Serialize;
use serde::Deserialize;

use std::error;
use std::string::String;

use crate::worldobject::human::actions::interact_action;
use crate::worldobject::human::controllers::terminal::TerminalHumanController;
use crate::quantities::duration;
use crate::worldobject;
use crate::world;
use crate::worldobject::components::inventory::{
    Inventory,
    item::{
        InventoryItem,
        none::NoInventoryItem
    }
};

use crate::quantities;
use crate::quantities::mass;
use crate::quantities::speed;
use crate::quantities::direction;
use crate::quantities::force;
use crate::quantities::distance;

use crate::worldobject::human::body::arm;
use crate::worldobject::TypedWorldObject;

#[derive(Serialize)]
pub struct UnsouledHuman {
    // identity
    name: String,
    gender: gender::Gender,

    // body
    mass: quantities::Quantity<mass::Mass>,

    // legs
    speed: quantities::Quantity<speed::Speed>,

    // arms
    dominant_arm: direction::DirectionHorizontal,
    arm_left: Option<arm::Arm>,
    arm_right: Option<arm::Arm>,

    // state
    inventory: Inventory,
}

impl UnsouledHuman {
    pub fn new(
        name: String,
        gender: gender::Gender,
        speed: quantities::Quantity<speed::Speed>,
        arm_left: Option<arm::Arm>,
        arm_right: Option<arm::Arm>,
        dominant_arm: direction::DirectionHorizontal,
        mass: quantities::Quantity<mass::Mass>,
        inventory: Inventory,
    ) -> UnsouledHuman {
        UnsouledHuman {
            name,
            mass,
            gender,
            speed,
            arm_left,
            arm_right,
            dominant_arm,
            inventory,
        }
    }
}

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

impl worldobject::TypedWorldObject for UnsouledHuman {
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

    fn collect(self: Box<Self>) -> Result<NoInventoryItem, (worldobject::Error, Box<Self>)> {
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
                |arm| <arm::Arm as TypedWorldObject>::dummy(&arm)
            ),
            arm_right: self.arm_right.as_ref().map(
                |arm| <arm::Arm as TypedWorldObject>::dummy(&arm)
            ),
            inventory: self.inventory.dummy()
        }
    }

    fn examine(&self) -> String {
        format!("a human {}; {} name is {}", self.gender.noun(), self.gender.possessive_pronoun(), self.name)
    }

    fn inventory(&self) -> Result<&Inventory, worldobject::Error> {
        Ok(&self.inventory)
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, worldobject::Error> {
        Ok(&mut self.inventory)
    }

    fn interact(&mut self) -> Result<String, worldobject::Error> {
        Ok(format!("{} says \"Hello\".", self.name))
    }

    /*
    fn interact(&self, actor: &world::WorldObjectHandle, world: &mut world::World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    */

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<worldobject::UpdateFn, worldobject::Error> {
        Ok(worldobject::UpdateFn::no_op())
    }

    fn send_message(&mut self, message: String) -> Result<(), worldobject::Error> {
        Ok(())
    }

    fn apply_force(&mut self, f: &quantities::Quantity<force::Force>) -> Result<String, worldobject::Error> {
        Ok(format!("{}'s .", self.definite_description()))
    }

    fn mass(&self) -> quantities::Quantity<mass::Mass> {
        self.mass.clone()
    }
}

impl TryFrom<&serde_json::Value> for UnsouledHuman {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let name = value.get("name").and_then(|v| v.as_str()).ok_or("name not found")?;
        let gender = value.get("gender").map(|v| gender::Gender::try_from(v)).transpose().map_err(|_| "gender not found")?.ok_or("gender not found")?;

        let speed = quantities::Quantity::<quantities::speed::Speed>::try_from(value.get("speed").cloned().ok_or("speed not found")?).map_err(|err| format!("failed to parse speed: {}", err))?;
        let arm_left = value.get("arm_left").map(|v| arm::Arm::try_from(v)).transpose().map_err(|err| format!("failed to parse arm_left: {}", err))?;
        let arm_right = value.get("arm_right").map(|v| arm::Arm::try_from(v)).transpose().map_err(|err| format!("failed to parse arm_right: {}", err))?;
        let dominant_arm = value.get("dominant_arm").map(|v| direction::DirectionHorizontal::try_from(v)).transpose().map_err(|err| format!("failed to parse dominant_arm: {}", err))?.ok_or("dominant_arm not found")?;
        let mass = value.get("mass").ok_or(String::from("mass not found"))
            .and_then(|value| quantities::Quantity::<mass::Mass>::try_from(value.clone()).map_err(|err| format!("failed to parse mass: {}", err)))?;

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
}

impl error::Error for HumanCollectError {}

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
/*
impl<'de> serde::Deserialize<'de> for UnsouledHuman {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;

        let name = value.get("name")
            .ok_or(D::Error::custom("name not found"))
            .and_then(
                |v| v.as_str().map(
                    |v| String::from(v)
                ).ok_or(D::Error::custom("invalid name"))
            )?;

        let gender = value.get("gender").map(|v| gender::Gender::try_from(v))
            .unwrap_or(Ok(gender::Gender::Other)).map_err(|err| D::Error::custom(err.to_string()))?;

        let speed = speed::meters_per_second(value.get("speed")
            .and_then(|v| v.as_f64())
            .ok_or(D::Error::custom("speed not found"))?);

        let arm_left = value.get("arm_left").map(|v| arm::Arm::try_from(v)).transpose().map_err(|err| D::Error::custom(err.to_string()))?;
        let arm_right = value.get("arm_right").map(|v| arm::Arm::try_from(v)).transpose().map_err(|err| D::Error::custom(err.to_string()))?;
        let dominant_arm = value.get("dominant_arm")
            .map(
                |v| direction::DirectionHorizontal::try_from(v)
                    .map_err(|err| D::Error::custom(err.to_string()))
            ).unwrap_or(Ok(direction::DirectionHorizontal::Right))?;

        let mass = mass::kilograms(value.get("mass").and_then(|v| v.as_f64()).ok_or(D::Error::custom("mass not found"))?);

        Ok(UnsouledHuman::new(name, gender, speed, arm_left, arm_right, dominant_arm, mass, Inventory::new()))
    }
}

impl serde::Serialize for UnsouledHuman {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        
    }
}
*/

impl std::error::Error for AttackError {}

impl worldobject::TypedWorldObject for Human {
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

    fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (worldobject::Error, Box<Self>)> {
        Err((Box::new(HumanCollectError()), self))
    }

    fn dummy(&self) -> Self::Dummy {
        self.unsouled.dummy()
    }

    fn examine(&self) -> String {
        format!("a human {}; {} name is {}", self.unsouled.gender.noun(), self.unsouled.gender.possessive_pronoun(), self.unsouled.name)
    }

    fn inventory(&self) -> Result<&Inventory, worldobject::Error> {
        Ok(&self.unsouled.inventory)
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, worldobject::Error> {
        Ok(&mut self.unsouled.inventory)
    }

    fn interact(&mut self) -> Result<String, worldobject::Error> {
        Ok(format!("{} says \"Hello\".", self.unsouled.name))
    }

    /*
    fn interact(&self, actor: &world::WorldObjectHandle, world: &mut world::World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    */

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<worldobject::UpdateFn, worldobject::Error> {
        let action = self.controller.prompt_turn(world)?;

        match action {
            actions::HumanAction::Move(actions::move_action::MoveAction {
                direction,
                distance: dist,
            }) => {
                if dist > (duration::seconds(1.0) * self.unsouled.speed.commute()).associate_left().commute().cancel() {
                    return Err(Box::new(HumanUpdateError::MoveError(MoveError::DistanceTooGreat)));
                }

                Ok(worldobject::UpdateFn(Box::new(move |world: &mut world::World| -> Result<Option<String>, worldobject::Error> {
                    world.move_object(&my_handle, &direction, &dist)
                        .map_err(|err| Box::new(err))?;

                    let dist_f64 = (&dist / &distance::meters(1.0)).cancel().0.0;

                    Ok(Some(format!("you move {} meters {}", dist_f64, direction)))
                })))
            }
            actions::HumanAction::Examine(actions::examine_action::ExamineAction {
                target_handle
            }) => {
                {
                    let description = world.get_object(&target_handle)
                        .map(|object| object.examine())
                        .map_err(|err| Box::new(err))?;
                    
                    self.controller.display_message(description)?;
                }
                
                Ok(worldobject::UpdateFn::no_op())
            },
            actions::HumanAction::Collect(actions::collect_action::CollectAction {
                target_handle
            }) => Ok(worldobject::UpdateFn(Box::new(
                move |world: &mut world::World| {
                    let location = world.locate_object(&target_handle)?;
                    let object = world.take_object(&target_handle)?;

                    let object_desc = object.definite_description();

                    let inventory_item = object.collect()
                        .or_else(|(err, og_object)| {
                            world.add_object(my_handle.clone(), og_object, location);
                            Err(err)
                    })?;

                    world.give_item_to(&my_handle, inventory_item)
                        .map_err(|err| Box::new(err))?;
                    
                    Ok(Some(format!("you collect {}", object_desc)))
                }
            ))),
            actions::HumanAction::Circumspect => {
                let descs = world.objects.iter()
                    .map(|(handle, (_, object))| format!("- {}: {}", handle, object.examine()))
                    .collect::<Vec<String>>();

                let message = if descs.is_empty() {
                    "You look around and see nothing.".to_string()
                } else {
                    format!("You look around and see the following:\n\t{}", descs.join("\n\t"))
                };

                self.controller.display_message(message)?;
                
                Ok(worldobject::UpdateFn::no_op())
            },
            actions::HumanAction::Attack(actions::attack_action::AttackAction {
                left_or_right_arm,
                target_handle
            }) => {
                let arm = match left_or_right_arm.as_ref().unwrap_or(&self.unsouled.dominant_arm) {
                    direction::DirectionHorizontal::Left => &self.unsouled.arm_left,
                    direction::DirectionHorizontal::Right => &self.unsouled.arm_right,
                }.as_ref().ok_or(HumanUpdateError::AttackError(AttackError::NoArmProvided))?;

                let punch_force = arm.punch_force.clone();

                Ok(worldobject::UpdateFn(Box::new(
                    move |world: &mut world::World| {
                        let object = world.get_object_mut(&target_handle)
                            .map_err(|err| Box::new(err))?;

                        let object_desc = object.definite_description();

                        let msg = object.apply_force(&punch_force)
                            .unwrap_or_else(|err| format!("failed to apply force: {}", err));

                        Ok(Some(format!("you punch {}; {}", object_desc, msg)))
                    }
                )))
            },
            actions::HumanAction::Interact(interact_action::InteractAction{
                target_handle
            }) => {
                Ok(worldobject::UpdateFn(Box::new(
                    move |world: &mut world::World| {
                        let object = world.get_object_mut(&target_handle)
                        .map_err(|err| Box::new(err))?;
            
                        let msg = object.interact()?;

                        Ok(Some(format!("you pet {}; {}", object.definite_description(), msg)))
                    }
                )))
            }
            actions::HumanAction::Inventory => {
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

                self.controller.display_message(message)?;

                Ok(worldobject::UpdateFn::no_op())
            },
        }
    }

    fn send_message(&mut self, message: String) -> Result<(), worldobject::Error> {
        self.controller.display_message(message)?;
        Ok(())
    }

    fn apply_force(&mut self, f: &quantities::Quantity<force::Force>) -> Result<String, worldobject::Error> {
        let force_newtons = (f / &force::newtons(1.0)).cancel().0.0;
        self.controller.display_message(format!("a force makes impact with you; it feels like roughly {} newtons.  Your hefty constitution absorbs the force.", force_newtons))?;
        Ok(format!("{}'s hefty constitution absorbs the force.", self.definite_description()))
    }

    fn mass(&self) -> quantities::Quantity<mass::Mass> {
        self.unsouled.mass.clone()
    }
}
