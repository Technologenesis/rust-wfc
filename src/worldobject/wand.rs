use std::error::Error as StdError;

use async_trait::async_trait;

use crate::{
    quantities::{
        Quantity,
        mass::{Mass, grams},
        force::Force,
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

pub struct Wand;

#[async_trait]
impl WorldObject for Wand {
    async fn update(&self, _my_handle: &WorldObjectHandle, _world: &World) -> Result<Action, Box<dyn StdError>> {
        Ok(Action::no_op())
    }

    fn linguistics(&self) -> WorldObjectLinguistics {
        WorldObjectLinguistics {
            name: String::from("wand"),
            definite_description: String::from("the wand"),
            indefinite_description: String::from("a wand"),
            pronoun: String::from("it"),
        }
    }

    async fn send_message(&mut self, _message: String) -> Result<(), Box<dyn StdError>> {
        Ok(())
    }

    fn as_controllable(self: Box<Self>) -> Result<Controllable, Box<dyn StdError>> {
        Err(Box::from("wand is not controllable"))
    }

    fn as_containable(self: Box<Self>) -> Result<Containable, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_container(self: Box<Self>) -> Result<Container, Box<dyn StdError>> {
        Err(Box::from("wand is not a container"))
    }

    fn as_person(self: Box<Self>) -> Result<Person, Box<dyn StdError>> {
        Err(Box::from("wand is not a person"))
    }

    fn as_physics_object(self: Box<Self>) -> Result<PhysicsObject, Box<dyn StdError>> {
        Ok(self)
    }

    fn as_wielder(self: Box<Self>) -> Result<Wielder, Box<dyn StdError>> {
        Err(Box::from("wand is not a wielder"))
    }

    fn as_wieldable(self: Box<Self>) -> Result<Wieldable, Box<dyn StdError>> {
        Ok(self)
    }
}

impl crate::worldobject::components::physics::PhysicsObjectTrait for Wand {
    fn mass(&self) -> Quantity<Mass> {
        grams(100.0)
    }

    fn apply_force(&self, _force: &Quantity<Force>) -> Result<String, Box<dyn StdError>> {
        Ok(String::from("the wand resists the force with incredible strength"))
    }
}

impl crate::worldobject::components::container::containable::ContainableTrait for Wand {}

impl crate::worldobject::components::wielder::wieldable::WieldableTrait for Wand {
    fn wieldable_name(&self) -> String {
        String::from("wand")
    }

    fn as_usable(&mut self) -> Option<&mut dyn crate::worldobject::components::wielder::usable::UsableTrait> {
        Some(self)
    }
}

impl crate::worldobject::components::wielder::usable::UsableTrait for Wand {
    fn use_item(
        &mut self,
        _target: Option<&crate::world::handle::WorldObjectHandle>,
    ) -> Result<crate::worldobject::components::wielder::usable::UseEffect, Box<dyn StdError>> {
        use crate::lang::{VerbPhrase, TransitiveVerbPhrase, TransitiveVerb, verbs::ToCast};

        // Transmogrify logic (controller swap) is a stub pending controller mechanism finalization
        Ok(crate::worldobject::components::wielder::usable::UseEffect {
            verb_phrase: VerbPhrase::Transitive(TransitiveVerbPhrase {
                verb: TransitiveVerb::new(ToCast),
                direct_object: String::from("a spell"),
            }),
            message: String::from("you wave the wand and a shower of sparks erupts from its tip"),
        })
    }
}
