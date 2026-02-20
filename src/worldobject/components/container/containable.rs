use crate::worldobject::WorldObject;

pub type Containable = Box<dyn ContainableTrait>;

pub trait ContainableTrait: WorldObject + Send + Sync {}