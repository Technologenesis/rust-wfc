use std::error::Error as StdError;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::{
    world::{World, handle::WorldObjectHandle},
    worldobject::{
        WorldObject,
        linguistics::WorldObjectLinguistics,
        fns::update::Action,
        components::{
            container::{Container, containable::Containable},
            controllable::{Controllable, controller::Controller},
            person::Person,
            physics::PhysicsObject,
            wielder::{Wielder, wieldable::Wieldable},
        },
    },
};

/// A WorldObject wrapper that pairs a Controllable with a Controller.
///
/// On each `update`, prompts the controller for a command and dispatches
/// it to the underlying controllable via `handle_command`.
pub struct Controlled {
    inner: Mutex<Controllable>,
    controller: tokio::sync::Mutex<Controller>,
}

impl Controlled {
    pub fn new(inner: Controllable, controller: Controller) -> Self {
        Self {
            inner: Mutex::new(inner),
            controller: tokio::sync::Mutex::new(controller),
        }
    }
}

#[async_trait]
impl WorldObject for Controlled {
    async fn update(&self, my_handle: &WorldObjectHandle, world: &World) -> Result<Action, Box<dyn StdError>> {
        let cmd = self.controller.lock().await.prompt_turn().await?;

        let action = {
            let mut inner = self.inner.lock().map_err(|e| format!("inner mutex poisoned: {}", e))?;
            inner.handle_command(cmd, my_handle, world)?
        };

        Ok(action)
    }

    fn linguistics(&self) -> WorldObjectLinguistics {
        self.inner.lock()
            .expect("inner mutex poisoned")
            .linguistics()
    }

    async fn send_message(&mut self, message: String) -> Result<(), Box<dyn StdError>> {
        self.controller.get_mut().display_message(message).await
    }

    fn as_controllable(self: Box<Self>) -> Result<Controllable, Box<dyn StdError>> {
        Ok(self.inner.into_inner().expect("inner mutex poisoned"))
    }

    fn as_containable(self: Box<Self>) -> Result<Containable, Box<dyn StdError>> {
        self.inner.into_inner().expect("inner mutex poisoned").as_containable()
    }

    fn as_container(self: Box<Self>) -> Result<Container, Box<dyn StdError>> {
        self.inner.into_inner().expect("inner mutex poisoned").as_container()
    }

    fn as_person(self: Box<Self>) -> Result<Person, Box<dyn StdError>> {
        self.inner.into_inner().expect("inner mutex poisoned").as_person()
    }

    fn as_physics_object(self: Box<Self>) -> Result<PhysicsObject, Box<dyn StdError>> {
        self.inner.into_inner().expect("inner mutex poisoned").as_physics_object()
    }

    fn as_wielder(self: Box<Self>) -> Result<Wielder, Box<dyn StdError>> {
        self.inner.into_inner().expect("inner mutex poisoned").as_wielder()
    }

    fn as_wieldable(self: Box<Self>) -> Result<Wieldable, Box<dyn StdError>> {
        self.inner.into_inner().expect("inner mutex poisoned").as_wieldable()
    }
}
