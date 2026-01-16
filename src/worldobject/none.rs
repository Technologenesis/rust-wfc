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
        mass::{
            Mass,
            kilograms
        },
        force::Force
    }
};

use super::{
    TypedWorldObject,
    components::inventory::{
        Inventory,
        item::none::NoInventoryItem
    },
    fns::update::Action,
    Error
};

/// NoWorldObject is an empty type that implements TypedWorldObject;
/// it wraps the never type to ensure that it is never instantiated.
/// This can be used to guarantee that particular outcomes will never
/// occur.
pub struct NoWorldObject(!);

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

// implement the TypedWorldObject trait for NoWorldObject
// naturally, this should never actually be used,
// but rust requires us to provide an implementation
#[async_trait]
impl TypedWorldObject for NoWorldObject {
    type Dummy = Self;
    type CollectInventoryItem = NoInventoryItem;

    fn name(&self) -> String {
        String::from("nothing")
    }

    fn examine(&self) -> String {
        String::from("nothing")
    }

    fn definite_description(&self) -> String {
        String::from("nothing")
    }
    
    fn pronoun(&self) -> String {
        String::from("it")
    }

    fn dummy(&self) -> Self {
        NoWorldObject(self.0)
    }

    async fn update(&mut self, my_handle: WorldObjectHandle, world: &World) -> Result<Action, Error> {
        Ok(Action::no_op())
    }

    async fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (Error, Box<Self>)> {
        Ok(NoInventoryItem(self.0))
    }

    fn inventory(&self) -> Result<&Inventory, Error> {
        Err(Box::new(NoWorldObjectInventoryError))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, Error> {
        Err(Box::new(NoWorldObjectInventoryError))
    }

    fn mass(&self) -> Quantity<Mass> {
        kilograms(0.0)
    }

    async fn apply_force(&mut self, force: &Quantity<Force>) -> Result<String, Error> {
        Err(Box::new(NoWorldObjectForceApplicationError))
    }

    async fn send_message(&mut self, message: String) -> Result<(), Error> {
        Err(Box::new(NoWorldObjectMessageSendError))
    }

    async fn interact(&mut self) -> Result<String, Error> {
        Err(Box::new(NoWorldObjectInteractError))
    }
}