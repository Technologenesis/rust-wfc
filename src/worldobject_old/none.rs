// This module contains the NoWorldObject type;
// It is a wrapper around the never type that implements
// the TypedWorldObject trait.
//
// This is useful when we want to guarantee that a WorldObject
// will not be yielded by a particular operation - for example,
// humans can't be collected, so their collect method returns
// a Result<NoWorldObject, ...>, guaranteeing that the result
// will always be an error.

use async_trait::async_trait;

use crate::{
    world::{
        World,
        handle::WorldObjectHandle
    },
    quantities::{
        Quantity,
        mass::Mass,
        force::Force
    }
};

use super::{
    WorldObject,
    components::{
        inventory::{
            Inventory,
            item::InventoryItem
        },
        controllers::Controller
    },
    fns::update::Action,
    Error
};

/// NoWorldObject is an empty type that implements TypedWorldObject;
/// it wraps the never type to ensure that it is never instantiated.
/// This can be used to guarantee that particular outcomes will never
/// occur.
pub struct NoWorldObject(pub !);

#[derive(Debug)]
pub struct NoWorldObjectInventoryError;

impl std::fmt::Display for NoWorldObjectInventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this object doesn't exist; much less does it have its own inventory")
    }
}

impl std::error::Error for NoWorldObjectInventoryError {}

#[derive(Debug)]
pub struct NoWorldObjectForceApplicationError;

impl std::fmt::Display for NoWorldObjectForceApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this object doesn't exist; no force can be applied to it")
    }
}

impl std::error::Error for NoWorldObjectForceApplicationError {}

#[derive(Debug)]
pub struct NoWorldObjectMessageSendError;

impl std::fmt::Display for NoWorldObjectMessageSendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this object doesn't exist; no message can be sent to it")
    }
}

impl std::error::Error for NoWorldObjectMessageSendError {}

#[derive(Debug)]
pub struct NoWorldObjectInteractError;

impl std::fmt::Display for NoWorldObjectInteractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this object doesn't exist; no interaction can be performed with it")
    }
}

impl std::error::Error for NoWorldObjectInteractError {}

#[derive(Debug)]
pub struct NoWorldObjectControllerError;

impl std::fmt::Display for NoWorldObjectControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this object doesn't exist; no controller can be set for it")
    }
}

impl std::error::Error for NoWorldObjectControllerError {}

// implement the TypedWorldObject trait for NoWorldObject
// naturally, this should never actually be used,
// but rust requires us to provide an implementation
#[async_trait]
impl WorldObject for NoWorldObject {
    fn name(&self) -> String {
        self.0
    }

    fn examine(&self) -> String {
        self.0
    }

    fn definite_description(&self) -> String {
        self.0
    }

    fn indefinite_description(&self) -> String {
        self.0
    }
    
    fn pronoun(&self) -> String {
        self.0
    }

    fn dummy(&self) -> Box<dyn WorldObject> {
        self.0
    }

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, Error> {
        self.0
    }

    async fn collect(self: Box<Self>) -> Result<Box<dyn InventoryItem>, (Error, Box<dyn WorldObject>)> {
        self.0
    }

    fn inventory(&self) -> Result<&Inventory, Error> {
        self.0
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, Error> {
        self.0
    }

    fn mass(&self) -> Quantity<Mass> {
        self.0
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, Error> {
        self.0
    }

    async fn send_message(&mut self, message: String) -> Result<(), Error> {
        self.0
    }

    async fn interact(&mut self) -> Result<String, Error> {
        self.0
    }

    fn controller(&self) -> Result<&dyn Controller, Error> {
        self.0
    }

    fn controller_mut(&mut self) -> Result<&mut dyn Controller, Error> {
        self.0
    }

    fn take_controller(&mut self) -> Result<Box<dyn Controller>, Error> {
        self.0
    }

    fn set_controller(&mut self, controller: Box<dyn Controller>) -> Result<(), (Box<dyn Controller>, Error)> {
        self.0
    }
}