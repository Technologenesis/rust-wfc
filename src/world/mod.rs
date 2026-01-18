pub mod coord;
pub mod handle;

use std::collections::{HashMap};
use std::vec;
use std::fmt;
use std::error;

use crate::{
    worldobject::fns::update::Action,
    lang::{GrammaticalPerson, TransitiveVerb, verbs::ToDo, TransitiveVerbPhrase, VerbPhrase},
    worldobject::{WorldObject, fns::Error as WorldObjectError, components::inventory::item::InventoryItem},
    quantities::{Quantity, distance::Distance, direction::DirectionHorizontalOrVertical},
    logging::{Logger, LoggerImpl, DynLogger, noop::NoopLogger}
};

use handle::WorldObjectHandle;
use coord::WorldCoord;

pub struct World {
    logger: DynLogger,
    pub objects: HashMap<WorldObjectHandle, (WorldCoord, Box<dyn WorldObject>)>,
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
    CoultNotGetInventory(WorldObjectHandle, WorldObjectError)
}

impl fmt::Display for WorldGiveItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchObject(handle) => write!(f, "no object found for handle \"{}\"", handle),
            Self::CoultNotGetInventory(handle, err) => write!(f, "could not get inventory for object \"{}\": {}", handle, err)
        }
    }
}

impl error::Error for WorldGiveItemError {}

impl World {
    pub fn new(logger: Logger<impl LoggerImpl + 'static>) -> Self {
        World {
            logger: logger.to_dyn(),
            objects: HashMap::new(),
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

    async fn broadcast(&mut self, message: String) -> Result<(), WorldObjectSendMessageError> {
        self.broadcast_by_recipient(|handle| message.clone()).await
    }

    async fn broadcast_by_recipient(&mut self, message_by_recipient: impl Fn(WorldObjectHandle) -> String) -> Result<(), WorldObjectSendMessageError> {
        for (handle, (_, object)) in self.objects.iter_mut() {
            _ = object.send_message(message_by_recipient(handle.clone())).await;
        }
        Ok(())
    }

    fn second_and_third_person_messages(third_person_subject: String, update_verb_phrase: VerbPhrase) -> (String, String) {
        let second_person_message = format!(
            "you {}",
            update_verb_phrase.conjugate(&GrammaticalPerson::SecondPersonSingular)
        );

        let third_person_message = format!(
            "{} {}",
            third_person_subject,
            update_verb_phrase.conjugate(&GrammaticalPerson::ThirdPersonSingularGendered)
        );

        (second_person_message, third_person_message)
    }

    async fn update_object(&mut self, handle: &WorldObjectHandle, object_description: String) -> Result<(), WorldUpdateError> {
        // broadcast the start of the turn
        _ = self.broadcast_by_recipient(|recipient_handle| {
            if recipient_handle == *handle {
                String::from("It is your turn to act")
            } else {
                format!("It is {}'s turn to act", object_description)
            }
        }).await;
    
        let action_res = {
            // create a dummy world to avoid double-borrowing the acting object
            let world_dummy = self.dummy();
            // try getting the object; if we succeed, call the object's update method.
            // This borrows the object from the world, necessitating the dummy world
            match self.get_object_mut(&handle) {
                Ok(object) => object.update(handle.clone(), &world_dummy).await,
                Err(err) => {
                    let b: Box<dyn std::error::Error> = Box::new(err);
                    Err(b)
                }
            }
        };
    
        let (second_person_message, third_person_message) = match action_res {
            Ok(action) => {
                let object_description = object_description.clone();
                async {
                    let action_verb_phrase = action.verb_phrase.clone();
                    
                    let message = action.call(self).await?;
                    
                    let (mut second_person_message, third_person_message) = Self::second_and_third_person_messages(
                        object_description,
                        action_verb_phrase
                    );
                
                    if let Some(message) = message {
                        second_person_message = format!("{}; {}", second_person_message, message);
                    }
                
                    Ok((second_person_message, third_person_message))
                }.await
            },
            Err(err) => Err(err)
        }.unwrap_or_else(
            |err| {
                let (second_person_message, third_person_message) = Self::second_and_third_person_messages(
                    object_description,
                    VerbPhrase::Transitive(
                        TransitiveVerbPhrase {
                            verb: TransitiveVerb::new(ToDo),
                            direct_object: String::from("nothing"),
                        }
                    )
                );
                (format!("{}; {}", err, second_person_message), third_person_message)
            }
        );
    
        // broadcast the results of the action to all objects in the world
        self.broadcast_by_recipient(|recipient_handle| {
            if recipient_handle == *handle {
                second_person_message.clone()
            } else {
                third_person_message.clone()
            }
        }).await.map_err(|err| WorldUpdateError::ObjectUpdateFailed(handle.clone(), Box::new(err)))
    }

    pub async fn update(&mut self) -> Result<(), WorldUpdateError> {
        self.logger.info(String::from("Updating world...")).await;

        let handles = self.objects.iter().map(
            |(handle, obj)|
            (handle.clone(), obj.1.definite_description())
        ).collect::<Vec<_>>();

        for (handle, object_description) in handles {
            match self.update_object(&handle, object_description).await {
                Ok(_) => self.logger.info(format!("Updated object with handle {}...", handle)).await,
                Err(err) => self.logger.error(format!("Failed to update object with handle {}: {}", handle, err)).await,
            };
            let _ = self.broadcast(String::from("\n")).await;
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