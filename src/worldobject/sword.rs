use std::error::Error as StdError;

use async_trait::async_trait;

use crate::{
    materials::Material,
    quantities::{
        Quantity,
        mass::{Mass, grams},
        force::Force,
        distance::{Distance, meters},
    },
    world::{World, handle::WorldObjectHandle},
    worldobject::{
        WorldObject,
        linguistics::WorldObjectLinguistics,
        fns::update::Action,
        components::{
            container::{Container, containable::Containable},
            controllable::Controllable,
            person::Person,
            physics::PhysicsObject,
            wielder::{Wielder, wieldable::Wieldable},
        },
    },
};

pub struct Sword {
    mass: Quantity<Mass>,
    reach: Quantity<Distance>,
    material: Material,
}

impl Sword {
    pub fn new(reach: Quantity<Distance>, material: Material) -> Sword {
        Sword { mass: grams(1500.0), reach, material }
    }
}

#[async_trait]
impl WorldObject for Sword {
    async fn update(&self, _my_handle: &WorldObjectHandle, _world: &World) -> Result<Action, Box<dyn StdError>> {
        Ok(Action::no_op())
    }

    fn linguistics(&self) -> WorldObjectLinguistics {
        WorldObjectLinguistics {
            name: String::from("sword"),
            definite_description: String::from("the sword"),
            indefinite_description: String::from("a sword"),
            pronoun: String::from("it"),
        }
    }

    async fn send_message(&mut self, _message: String) -> Result<(), Box<dyn StdError>> {
        Ok(())
    }

    fn as_controllable(self: Box<Self>) -> Result<Controllable, Box<dyn StdError>> {
        Err(Box::from("sword is not controllable"))
    }

    fn as_containable(self: Box<Self>) -> Result<Containable, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_container(self: Box<Self>) -> Result<Container, Box<dyn StdError>> {
        Err(Box::from("sword is not a container"))
    }

    fn as_person(self: Box<Self>) -> Result<Person, Box<dyn StdError>> {
        Err(Box::from("sword is not a person"))
    }

    fn as_physics_object(self: Box<Self>) -> Result<PhysicsObject, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_wielder(self: Box<Self>) -> Result<Wielder, Box<dyn StdError>> {
        Err(Box::from("sword is not a wielder"))
    }

    fn as_wieldable(self: Box<Self>) -> Result<Wieldable, Box<dyn StdError>> {
        Ok(self)
    }
}

impl crate::worldobject::components::physics::PhysicsObjectTrait for Sword {
    fn mass(&self) -> Quantity<Mass> {
        self.mass.clone()
    }

    fn apply_force(&self, _force: &Quantity<Force>) -> Result<String, Box<dyn StdError>> {
        Ok(String::from("the sword bends with the force, but recovers its shape"))
    }
}

impl crate::worldobject::components::container::containable::ContainableTrait for Sword {}

impl crate::worldobject::components::wielder::wieldable::WieldableTrait for Sword {
    fn wieldable_name(&self) -> String {
        String::from("sword")
    }

    fn as_usable(&mut self) -> Option<&mut dyn crate::worldobject::components::wielder::usable::UsableTrait> {
        Some(self)
    }
}

impl crate::worldobject::components::wielder::usable::UsableTrait for Sword {
    fn use_item(
        &mut self,
        _target: Option<&crate::world::handle::WorldObjectHandle>,
    ) -> Result<crate::worldobject::components::wielder::usable::UseEffect, Box<dyn StdError>> {
        use crate::lang::{VerbPhrase, TransitiveVerbPhrase, TransitiveVerb, verbs::ToUse};

        Ok(crate::worldobject::components::wielder::usable::UseEffect {
            verb_phrase: VerbPhrase::Transitive(TransitiveVerbPhrase {
                verb: TransitiveVerb::new(ToUse),
                direct_object: String::from("the sword"),
            }),
            message: String::from("you swing the sword through the air with a sharp whistle"),
        })
    }
}
