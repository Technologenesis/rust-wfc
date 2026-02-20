pub mod containable;

use std::error::Error as StdError;

use crate::worldobject::WorldObject;

use containable::Containable;

pub type ContainerHandle = u16;

pub type Container = Box<dyn ContainerTrait>;

pub trait ContainerTrait: WorldObject {
    fn contents(&self) -> Vec<(ContainerHandle, Containable)>;
    fn add(&mut self, item: Containable) -> Result<ContainerHandle, Box<dyn StdError>>;

    fn get(&self, handle: ContainerHandle) -> Result<Containable, Box<dyn StdError>>;
    fn get_mut(&mut self, handle: ContainerHandle) -> Result<Containable, Box<dyn StdError>>;
    fn take(&mut self, handle: ContainerHandle) -> Result<Containable, Box<dyn StdError>>;
}