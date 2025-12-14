use std::collections;
use std::vec;
use std::fmt;
use std::error;

use crate::quantities;
use crate::worldobject;
use crate::components::inventory::InventoryItem;
use crate::quantities::distance;
use crate::quantities::direction;

#[derive(Debug, Clone, Copy)]
pub struct WorldCoord {
    pub x: quantities::Quantity<distance::Distance>,
    pub y: quantities::Quantity<distance::Distance>
}

impl WorldCoord {
    pub fn new(x: quantities::Quantity<distance::Distance>, y: quantities::Quantity<distance::Distance>) -> WorldCoord {
        WorldCoord { x, y }
    }

    fn translate_direction(&mut self, direction: &direction::DirectionHorizontalOrVertical, distance: &quantities::Quantity<distance::Distance>) {
        match &direction {
            &direction::DirectionHorizontalOrVertical::Vertical(vertical_direction) => match vertical_direction {
                direction::DirectionVertical::Up => self.y = &self.y + distance,
                direction::DirectionVertical::Down => self.y = &self.y - distance
            },
            &direction::DirectionHorizontalOrVertical::Horizontal(horizontal_direction) => match horizontal_direction {
                direction::DirectionHorizontal::Right => self.x = &self.x + distance,
                direction::DirectionHorizontal::Left => self.x = &self.x - distance
            },
        }
    }
}

pub type WorldObjectHandle = String;

pub struct World {
    pub objects: collections::HashMap<WorldObjectHandle, (WorldCoord, Box<dyn worldobject::WorldObject>)>,
}

#[derive(Debug)]
pub enum WorldUpdateError {
    ObjectUpdateFailed(WorldObjectHandle, worldobject::Error),
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
}

impl fmt::Display for WorldObjectSendMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchObject(handle) => write!(f, "no object found for handle \"{}\"", handle)
        }
    }
}

impl error::Error for WorldObjectSendMessageError {}

#[derive(Debug)]
pub enum WorldGiveItemError {
    NoSuchObject(WorldObjectHandle),
    CoultNotGetInventory(WorldObjectHandle, worldobject::Error),
    CouldNotGiveItem(WorldObjectHandle, worldobject::Error)
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
    pub fn new() -> Self {
        World {
            objects: collections::HashMap::new(),
        }
    }

    pub fn add_object(&mut self, handle: WorldObjectHandle, object: Box<dyn worldobject::WorldObject>, position: WorldCoord) {
        self.objects.insert(handle, (position, object));
    }

    pub fn get_object(&self, handle: &WorldObjectHandle) -> Result<&dyn worldobject::WorldObject, WorldObjectGetError> {
        let (_, obj_box) = self.objects.get(handle)
            .ok_or(WorldObjectGetError::NoSuchObject(handle.clone()))?;

        Ok(obj_box.as_ref())
    }

    pub fn get_object_mut(&mut self, handle: &WorldObjectHandle) -> Result<&mut dyn worldobject::WorldObject, WorldObjectGetError> {
        let (_, obj_box) = self.objects.get_mut(handle)
            .ok_or(WorldObjectGetError::NoSuchObject(handle.clone()))?;

        Ok(obj_box.as_mut())
    }

    pub fn locate_object(&self, handle: &WorldObjectHandle) -> Result<WorldCoord, WorldObjectGetError> {
        let (coord, _) = self.objects.get(handle)
            .ok_or(WorldObjectGetError::NoSuchObject(handle.clone()))?;

        Ok(*coord)
    }

    pub fn take_object(&mut self, handle: &WorldObjectHandle) -> Result<Box<dyn worldobject::WorldObject>, WorldObjectGetError> {
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

    pub fn send_message_to(&mut self, handle: &WorldObjectHandle, message: String) -> Result<(), WorldObjectSendMessageError> {
        let (_, object) = self.objects.get_mut(handle)
            .ok_or(WorldObjectSendMessageError::NoSuchObject(handle.clone()))?;

        object.send_message(message);

        Ok(())
    }

    pub fn move_object(&mut self, handle: &WorldObjectHandle, direction: &direction::DirectionHorizontalOrVertical, distance: &quantities::Quantity<distance::Distance>) -> Result<(), WorldObjectMoveError> {
        if !self.objects.contains_key(handle) {
            return Err(WorldObjectMoveError::NoSuchObject(handle.clone()))
        }

        self.objects
            .entry(handle.clone())
            .and_modify(|(coord, _)| coord.translate_direction(direction, distance));

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), WorldUpdateError> {
        let mut update_fns = vec!();

        let world_clone = self.dummy();
        for (handle, (_, object)) in self.objects.iter_mut() {
            update_fns.push((handle.clone(), object.update(handle.clone(), &world_clone)
                .map_err(|e| WorldUpdateError::ObjectUpdateFailed(handle.clone(), e))));
        }

        for (handle, update_fn_res) in update_fns {
            match update_fn_res {
                Ok(update_fn) => update_fn.call(self)
                    .map(|message| message.map(|message| self.send_message_to(&handle, message)))
                    .map_err(|err| self.send_message_to(&handle, format!("encountered an error while executing your action: {}", err))),
                Err(e) => return Err(e),
            };
        }

        Ok(())
    }

    pub fn dummy(&self) -> World {
        World {
            objects: self.objects.iter().map(|(handle, (coord, object))| (handle.clone(), (coord.clone(), object.dummy()))).collect(),
        }
    }
}