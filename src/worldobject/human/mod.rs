pub mod controllable;

use std::error::Error as StdError;
use async_trait::async_trait;
use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::{
    quantities::{
        Quantity,
        mass::Mass,
        force::Force,
        direction::DirectionHorizontal,
    },
    world::{
        World,
        handle::WorldObjectHandle
    },
    worldobject::{
        linguistics::WorldObjectLinguistics,
        WorldObject,
        components::{
            controllable::{Controllable, controller::Controller},
            person::{Person, PersonTrait, gender::Gender},
            physics::{PhysicsObject, PhysicsObjectTrait},
            container::{
                Container, ContainerTrait, ContainerHandle,
                containable::Containable
            },
            wielder::{
                Wielder, WielderTrait,
                wieldable::Wieldable
            }
        },
        fns::update::Action
    }
};

pub struct Human {
    // identity
    pub name: String,
    pub gender: Gender,

    // misc
    pub dominant_arm: DirectionHorizontal,

    // body
    pub body: crate::worldobject::components::physics::body::Body,
    pub inventory: Vec<Containable>,

    // controller
    pub controller: Option<Controller>,
}

#[derive(Debug)]
pub struct HumanCollectError ();

impl std::fmt::Display for HumanCollectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "humans can't be collected")
    }
}

impl StdError for HumanCollectError {}

#[derive(Debug)]
pub struct HumanNoControllerError ();

impl std::fmt::Display for HumanNoControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "human has no controller")
    }
}

impl StdError for HumanNoControllerError {}

impl Human {
    pub fn new(
        name: String,
        gender: Gender,
        body: crate::worldobject::components::physics::body::Body,
        dominant_arm: DirectionHorizontal,
        controller: Option<Controller>,
    ) -> Human {
        Human{
            name,
            gender,
            body,
            dominant_arm,
            inventory: Vec::new(),
            controller,
        }
    }

    pub fn set_controller(&mut self, controller: Controller) -> Result<(), (Controller, Box<dyn StdError>)> {
        self.controller = Some(controller);
        Ok(())
    }

    pub fn take_controller(&mut self) -> Result<Controller, Box<dyn StdError>> {
        self.controller.take().ok_or_else(|| -> Box<dyn StdError> { Box::new(HumanNoControllerError()) })
    }
}

#[async_trait]
impl WorldObject for Human {
    async fn update(&self, _my_handle: &WorldObjectHandle, _world: &World) -> Result<Action, Box<dyn StdError>> {
        Ok(Action::no_op())
    }

    fn linguistics(&self) -> WorldObjectLinguistics {
        WorldObjectLinguistics {
            name: self.name.clone(),
            definite_description: self.name.clone(),
            indefinite_description: format!("a {}", self.gender.noun()),
            pronoun: self.gender.subject_pronoun().to_string(),
        }
    }

    async fn send_message(&mut self, message: String) -> Result<(), Box<dyn StdError>> {
        match &mut self.controller {
            Some(controller) => controller.display_message(message).await,
            None => Ok(()),
        }
    }

    fn as_controllable(self: Box<Self>) -> Result<Controllable, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_containable(self: Box<Self>) -> Result<Containable, Box<dyn StdError>> {
        Err(Box::from(format!("{} is not a containable", self.linguistics().name)))
    }

    fn as_container(self: Box<Self>) -> Result<Container, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_person(self: Box<Self>) -> Result<Person, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_physics_object(self: Box<Self>) -> Result<PhysicsObject, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_wielder(self: Box<Self>) -> Result<Wielder, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_wieldable(self: Box<Self>) -> Result<Wieldable, Box<dyn StdError>> {
        Err(Box::from(format!("{} is not a wieldable", self.linguistics().name)))
    }
}

impl PersonTrait for Human {
    fn gender(&self) -> Option<Gender> {
        Some(self.gender)
    }
}

impl PhysicsObjectTrait for Human {
    fn mass(&self) -> Quantity<Mass> {
        self.body.base_mass.clone()
    }

    fn apply_force(&self, _force: &Quantity<Force>) -> Result<String, Box<dyn StdError>> {
        Ok(String::from("the force hits you"))
    }
}

impl WielderTrait for Human {
    fn wield(&mut self, item: Wieldable) -> Result<(), Box<dyn StdError>> {
        let wielding_arm = match self.dominant_arm {
            DirectionHorizontal::Left => &mut self.body.torso.left_arm,
            DirectionHorizontal::Right => &mut self.body.torso.right_arm,
        };
        wielding_arm.wield(item)
    }
}

impl ContainerTrait for Human {
    fn contents(&self) -> Vec<(ContainerHandle, Containable)> {
        todo!()
    }

    fn add(&mut self, _item: Containable) -> Result<ContainerHandle, Box<dyn StdError>> {
        todo!()
    }

    fn get(&self, _handle: ContainerHandle) -> Result<Containable, Box<dyn StdError>> {
        todo!()
    }

    fn get_mut(&mut self, _handle: ContainerHandle) -> Result<Containable, Box<dyn StdError>> {
        todo!()
    }

    fn take(&mut self, _handle: ContainerHandle) -> Result<Containable, Box<dyn StdError>> {
        todo!()
    }
}

impl Serialize for Human {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Human", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("gender", &self.gender)?;
        state.serialize_field("body", &self.body)?;
        state.serialize_field("dominant_arm", &self.dominant_arm)?;
        // inventory omitted: dyn ContainableTrait is not Serialize
        state.end()
    }
}

impl TryFrom<&serde_json::Value> for Human {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let name = value.get("name").and_then(|v| v.as_str()).ok_or("name not found")?;
        let gender = value.get("gender").map(|v| Gender::try_from(v)).transpose().map_err(|_| "gender not found")?.ok_or("gender not found")?;

        let dominant_arm = value.get("dominant_arm").map(|v| DirectionHorizontal::try_from(v)).transpose().map_err(|err| format!("failed to parse dominant_arm: {}", err))?.ok_or("dominant_arm not found")?;

        let body = crate::worldobject::components::physics::body::Body::try_from(value.get("body").ok_or("body not found")?).map_err(|err| format!("failed to parse body: {}", err))?;

        Ok(Human::new(String::from(name), gender, body, dominant_arm, None::<Controller>))
    }
}
