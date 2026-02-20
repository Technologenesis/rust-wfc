pub mod gender;

use crate::worldobject::WorldObject;

use gender::Gender;

pub type Person = Box<dyn PersonTrait>;

pub trait PersonTrait: WorldObject {
    fn gender(&self) -> Option<Gender>;
}