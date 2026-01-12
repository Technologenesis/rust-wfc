use crate::world;
use crate::worldobject;
use crate::worldobject::components::inventory::item::none;
use crate::worldobject::components::inventory;
use crate::quantities;
use crate::quantities::mass::{
    Mass,
    kilograms
};
use crate::quantities::force;

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
impl worldobject::TypedWorldObject for NoWorldObject {
    type Dummy = Self;
    type CollectInventoryItem = none::NoInventoryItem;

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

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<worldobject::UpdateFn, worldobject::Error> {
        Ok(worldobject::UpdateFn::no_op())
    }

    fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (worldobject::Error, Box<Self>)> {
        Ok(none::NoInventoryItem(self.0))
    }

    fn inventory(&self) -> Result<&inventory::Inventory, worldobject::Error> {
        Err(Box::new(NoWorldObjectInventoryError))
    }

    fn inventory_mut(&mut self) -> Result<&mut inventory::Inventory, worldobject::Error> {
        Err(Box::new(NoWorldObjectInventoryError))
    }

    fn mass(&self) -> quantities::Quantity<Mass> {
        kilograms(0.0)
    }

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, worldobject::Error> {
        Err(Box::new(NoWorldObjectForceApplicationError))
    }

    fn send_message(&mut self, message: String) -> Result<(), worldobject::Error> {
        Err(Box::new(NoWorldObjectMessageSendError))
    }

    fn interact(&mut self) -> Result<String, worldobject::Error> {
        Err(Box::new(NoWorldObjectInteractError))
    }
}