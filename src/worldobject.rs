use socket2::Type;

use crate::world;

use crate::components::inventory::{Inventory, InventoryItem, NoInventoryItem};
use crate::quantities;
use crate::quantities::mass;
use crate::quantities::force;

pub struct UpdateFn(pub Box<dyn FnOnce(&mut world::World) -> Result<Option<String>, Error>>);
pub type InteractFn = Box<dyn FnOnce(&mut world::World) -> Result<(), Error>>;
pub type Error = Box<dyn std::error::Error>;

impl UpdateFn {
    pub fn call(self, world: &mut world::World) -> Result<Option<String>, Error> {
        self.0(world)
    }

    pub fn no_op() -> Self {
        Self(Box::new(|_| Ok(None)))
    }
}

// TypedWorldObject is a trait similar to WorldObject,
// but with type parameters where WorldObject uses dyn
// trait objects.  Implementing this trait automatically
// implements the WorldObject trait, but may allow certain
// users of the type to rely on more specific type constraints.
pub trait TypedWorldObject: {
    type Dummy: WorldObject + Sized + 'static;
    type CollectInventoryItem: InventoryItem + Sized + 'static;

    // used as a hint when generating handles for this object
    fn name(&self) -> String;
    fn examine(&self) -> String;
    fn definite_description(&self) -> String;
    fn pronoun(&self) -> String;

    // creates a new object with the same properties as this one,
    // minus any fields that are not cloneable (typically controllers)
    fn dummy(&self) -> Self::Dummy;

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<UpdateFn, Error>;
    
    fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (Error, Box<Self>)>;

    fn inventory(&self) -> Result<&Inventory, Error>;
    fn inventory_mut(&mut self) -> Result<&mut Inventory, Error>;

    fn mass(&self) -> quantities::Quantity<mass::Mass>;

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, Error>;

    fn send_message(&mut self, message: String) -> Result<(), Error>;

    fn interact(&mut self) -> Result<String, Error>;
}

impl<T: TypedWorldObject + 'static> WorldObject for T {
    fn name(&self) -> String {
        <T as TypedWorldObject>::name(self)
    }

    fn examine(&self) -> String {
        <T as TypedWorldObject>::examine(self)
    }

    fn definite_description(&self) -> String {
        <T as TypedWorldObject>::definite_description(&self)
    }

    fn pronoun(&self) -> String {
        <T as TypedWorldObject>::pronoun(&self)
    }

    fn dummy(&self) -> Box<dyn WorldObject> {
        Box::new(<T as TypedWorldObject>::dummy(self))
    }

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<UpdateFn, Error> {
        <T as TypedWorldObject>::update(self, my_handle, world)
    }

    fn collect(self: Box<Self>) -> Result<Box<dyn InventoryItem<Dummy>>, (Error, Box<dyn WorldObject>)> {
        <T as TypedWorldObject>::collect(self)
            .map(|item| Box::new(item) as Box<dyn InventoryItem>)
            .map_err(|(err, obj)| (err, Box::new(obj) as Box<dyn WorldObject>))
    }

    fn inventory(&self) -> Result<&Inventory, Error> {
        <T as TypedWorldObject>::inventory(&self)
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, Error> {
        <T as TypedWorldObject>::inventory_mut(self)
    }

    fn mass(&self) -> quantities::Quantity<mass::Mass> {
        <T as TypedWorldObject>::mass(self)
    }

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, Error> {
        <T as TypedWorldObject>::apply_force(self, force)
    }

    fn send_message(&mut self, message: String) -> Result<(), Error> {
        <T as TypedWorldObject>::send_message(self, message)
    }

    fn interact(&mut self) -> Result<String, Error> {
        <T as TypedWorldObject>::interact(self)
    }
}

pub trait WorldObject {
    // used as a hint when generating handles for this object
    fn name(&self) -> String;
    fn examine(&self) -> String;
    fn definite_description(&self) -> String;
    fn pronoun(&self) -> String;

    // creates a new object with the same properties as this one,
    // minus any fields that are not cloneable (typically controllers)
    fn dummy(&self) -> Box<dyn WorldObject>;

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<UpdateFn, Error>;
    
    fn collect(self: Box<Self>) -> Result<Box<dyn InventoryItem>, (Error, Box<dyn WorldObject>)>;

    fn inventory(&self) -> Result<&Inventory, Error>;
    fn inventory_mut(&mut self) -> Result<&mut Inventory, Error>;

    fn mass(&self) -> quantities::Quantity<mass::Mass>;

    fn apply_force(&mut self, force: &quantities::Quantity<force::Force>) -> Result<String, Error>;

    fn send_message(&mut self, message: String) -> Result<(), Error>;

    fn interact(&mut self) -> Result<String, Error>;
}

/// NoWorldObject is an empty type that implements TypedWorldObject;
pub struct NoWorldObject(!);

// implement the TypedWorldObject trait for NoWorldObject
// naturally, this should never actually be used,
// but rust requires us to provide an implementation
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

    fn dummy(&self) -> Self::Dummy {
        Box::new(Self(self.0))
    }

    fn update(&mut self, my_handle: world::WorldObjectHandle, world: &world::World) -> Result<UpdateFn, Error> {
        Ok(UpdateFn::no_op())
    }

    fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (Error, Box<Self>)> {
        Ok(Box::new(NoInventoryItem(self.0)))
    }

    fn inventory(&self) -> Result<&Inventory, Error> {
        Err(Box::new(NoInventoryError()))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, Error> {
        Err(Box::new(NoInventoryError()))
    }

    fn mass(&self) -> quantities::Quantity<mass::Mass> {
        quantities::Quantity::zero()
    }
}