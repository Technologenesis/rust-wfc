use crate::worldobject::WorldObject;

pub type Collectible = Box<dyn CollectibleTrait>;

pub trait CollectibleTrait: WorldObject {}