pub mod coord;
pub mod handle;

use std::collections;
use std::vec;
use std::fmt;
use std::error;

use crate::{
    worldobject::{
        WorldObject,
        fns::Error as WorldObjectError,
        components::inventory::item::InventoryItem
    },
    quantities::{
        Quantity,
        distance::Distance,
        direction::DirectionHorizontalOrVertical
    },
    logging::{
        Logger,
        LoggerImpl,
        DynLogger,
        noop::NoopLogger
    }
};

use handle::WorldObjectHandle;
use coord::WorldCoord;

pub struct World {
    logger: DynLogger,
    pub objects: collections::HashMap<WorldObjectHandle, (WorldCoord, Box<dyn WorldObject>)>,
}

#[derive(Debug)]
pub enum WorldUpdateError {
    ObjectUpdateFailed(WorldObjectHandle, WorldObjectError),
}

impl fmt::Display for WorldUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ObjectUpdateFailed(handle, err) => write!(f, "failed to update object with handle \"{}\": {}", handle, err)
        }
    }
}

#[derive(Debug)]
pub enum WorldObjectMoveError {
    NoSuchObject(WorldObjectHandle)
}

impl fmt::Display for WorldObjectMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchObject(handle) => write!(f, "no object found for handle \"{}\"", handle)
        }
    }
}

impl error::Error for WorldObjectMoveError {}

#[derive(Debug)]
pub enum WorldObjectGetError {
    NoSuchObject(WorldObjectHandle),
}

impl fmt::Display for WorldObjectGetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchObject(handle) => write!(f, "no object found for handle \"{}\"", handle)
        }
    }
}

impl error::Error for WorldObjectGetError {}

#[derive(Debug)]
pub enum WorldObjectSendMessageError {
    NoSuchObject(WorldObjectHandle),
    ErrorSendingMessage(Box<dyn std::error::Error>)
}

impl fmt::Display for WorldObjectSendMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchObject(handle) => write!(f, "no object found for handle \"{}\"", handle),
            Self::ErrorSendingMessage(err) => write!(f, "error sending message: {}", err)
        }
    }
}

impl error::Error for WorldObjectSendMessageError {}

#[derive(Debug)]
pub enum WorldGiveItemError {
    NoSuchObject(WorldObjectHandle),
    CoultNotGetInventory(WorldObjectHandle, WorldObjectError),
    CouldNotGiveItem(WorldObjectHandle, WorldObjectError)
}

impl fmt::Display for WorldGiveItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchObject(handle) => write!(f, "no object found for handle \"{}\"", handle),
            Self::CoultNotGetInventory(handle, err) => write!(f, "could not get inventory for object \"{}\": {}", handle, err),
            Self::CouldNotGiveItem(handle, err) => write!(f, "could not give item to object \"{}\": {}", handle, err)
        }
    }
}

impl error::Error for WorldGiveItemError {}

impl World {
    pub fn new(logger: Logger<impl LoggerImpl + 'static>) -> Self {
        World {
            logger: logger.to_dyn(),
            objects: collections::HashMap::new(),
        }
    }

    pub fn add_object(&mut self, handle: WorldObjectHandle, object: Box<dyn WorldObject>, position: WorldCoord) {
        self.objects.insert(handle, (position, object));
    }

    pub fn get_object(&self, handle: &WorldObjectHandle) -> Result<&dyn WorldObject, WorldObjectGetError> {
        let (_, obj_box) = self.objects.get(handle)
            .ok_or(WorldObjectGetError::NoSuchObject(handle.clone()))?;

        Ok(obj_box.as_ref())
    }

    pub fn get_object_mut(&mut self, handle: &WorldObjectHandle) -> Result<&mut dyn WorldObject, WorldObjectGetError> {
        let (_, obj_box) = self.objects.get_mut(handle)
            .ok_or(WorldObjectGetError::NoSuchObject(handle.clone()))?;

        Ok(obj_box.as_mut())
    }

    pub fn locate_object(&self, handle: &WorldObjectHandle) -> Result<WorldCoord, WorldObjectGetError> {
        let (coord, _) = self.objects.get(handle)
            .ok_or(WorldObjectGetError::NoSuchObject(handle.clone()))?;

        Ok(*coord)
    }

    pub fn take_object(&mut self, handle: &WorldObjectHandle) -> Result<Box<dyn WorldObject>, WorldObjectGetError> {
        let (_, obj_box) = self.objects.remove(handle)
            .ok_or(WorldObjectGetError::NoSuchObject(handle.clone()))?;

        Ok(obj_box)
    }

    pub fn give_item_to<Item: InventoryItem + 'static>(&mut self, handle: &WorldObjectHandle, item: Item) -> Result<(), WorldGiveItemError> {
        let (_, object) = self.objects.get_mut(handle)
            .ok_or(WorldGiveItemError::NoSuchObject(handle.clone()))?;

        let inventory = object.inventory_mut()
            .map_err(|err| WorldGiveItemError::CoultNotGetInventory(String::from(handle), err))?;

        inventory.give(item);

        Ok(())
    }

    pub async fn send_message_to(&mut self, handle: &WorldObjectHandle, message: String) -> Result<(), WorldObjectSendMessageError> {
        self.logger.info(format!("Sending message to object with handle {}...", handle)).await;

        let (_, object) = self.objects.get_mut(handle)
            .ok_or(WorldObjectSendMessageError::NoSuchObject(handle.clone()))?;

        if let Err(err) = object.send_message(message).await {
            self.logger.error(format!("Failed to send message to object with handle {}: {}", handle, err)).await;
            return Err(WorldObjectSendMessageError::ErrorSendingMessage(err));
        }

        Ok(())
    }

    pub fn move_object(&mut self, handle: &WorldObjectHandle, direction: &DirectionHorizontalOrVertical, distance: &Quantity<Distance>) -> Result<(), WorldObjectMoveError> {
        if !self.objects.contains_key(handle) {
            return Err(WorldObjectMoveError::NoSuchObject(handle.clone()))
        }

        self.objects
            .entry(handle.clone())
            .and_modify(|(coord, _)| coord.translate_direction(direction, distance));

        Ok(())
    }

    pub async fn update(&mut self) -> Result<(), WorldUpdateError> {
        self.logger.info(String::from("Updating world...")).await;

        let mut update_fns = vec!();

        let world_dummy = self.dummy();
        for (handle, (_, object)) in self.objects.iter_mut() {
            self.logger.info(format!("Prompting object with handle {} for turn...", handle)).await;
            update_fns.push((handle.clone(), object.update(handle.clone(), &world_dummy).await));
        }

        for (handle, update_fn_res) in update_fns {
            match update_fn_res {
                Ok(update_fn) => {
                    self.logger.info(format!("calling update function for object with handle {}...", handle)).await;

                    let update_res = update_fn.call(self).await;
                    if let Ok(Some(message)) = update_res {
                        _ = self.send_message_to(&handle, message).await;
                    } else if let Err(err) = update_res {
                        self.logger.error(format!("Failed to run update function for object with handle {}: {}", handle, err)).await;
                        _ = self.send_message_to(&handle, err.to_string()).await;
                    }
                },
                Err(err) => {
                    self.logger.error(format!("Failed to prompt object with handle {} for turn: {}", handle, err)).await;
                    _ = self.send_message_to(&handle, err.to_string()).await;
                }
            };
        }

        Ok(())
    }

    pub fn dummy(&self) -> Self {
        Self {
            logger: NoopLogger::new().to_dyn(),
            objects: self.objects.iter().map(|(handle, (coord, object))| (handle.clone(), (coord.clone(), object.dummy()))).collect(),
        }
    }
}