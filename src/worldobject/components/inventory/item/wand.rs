use async_trait::async_trait;

use crate::lang::{VerbPhrase, TransitiveVerbPhrase, TransitiveVerb, PrepositionalVerbPhrase, PrepositionalPhrase, verbs::ToCast};
use crate::world::{World, handle::WorldObjectHandle};
use crate::worldobject::{
    TypedWorldObject,
    Error as WorldObjectError,
    fns::update::Action,
    components::{
        inventory::{
            Inventory,
            item::InventoryItem
        },
        controllers::Controller
    }
};
use crate::quantities::{
    Quantity,
    mass::{Mass, grams},
    force::Force
};

pub struct Wand;

#[derive(Debug)]
pub struct WandInventoryError;

impl std::error::Error for WandInventoryError {}

impl std::fmt::Display for WandInventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "wands don't have inventories")
    }
}

#[derive(Debug)]
pub struct WandControllerError;

impl std::fmt::Display for WandControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "wands don't have controllers")
    }
}

impl std::error::Error for WandControllerError {}

#[async_trait]
impl TypedWorldObject for Wand {
    type Dummy = Self;
    type CollectInventoryItem = Self;

    fn name(&self) -> String {
        String::from("wand")
    }

    fn examine(&self) -> String {
        String::from("a wand")
    }
    
    fn definite_description(&self) -> String {
        String::from("the wand")
    }

    fn indefinite_description(&self) -> String {
        String::from("a wand")
    }

    fn pronoun(&self) -> String {
        String::from("it")
    }

    fn dummy(&self) -> Self {
        Wand {}
    }

    fn inventory(&self) -> Result<&Inventory, WorldObjectError> {
        Err(Box::new(WandInventoryError))
    }

    fn inventory_mut(&mut self) -> Result<&mut Inventory, WorldObjectError> {
        Err(Box::new(WandInventoryError))
    }

    fn mass(&self) -> Quantity<Mass> {
        grams(100.0)
    }

    async fn apply_force(&mut self, _: &Quantity<Force>) -> Result<String, WorldObjectError> {
        Ok(String::from("the wand resists the force with incredible strength"))
    }

    async fn send_message(&mut self, _: String) -> Result<(), WorldObjectError> {
        Ok(())
    }

    async fn interact(&mut self) -> Result<String, WorldObjectError> {
        Ok(String::from("you can't think of anything particularly interesting to do with this."))
    }

    async fn update(&mut self, target_handle: WorldObjectHandle, world: &World) -> Result<Action, WorldObjectError> {
        Ok(Action::no_op())
    }

    async fn collect(self: Box<Self>) -> Result<Self::CollectInventoryItem, (WorldObjectError, Box<Self>)> {
        Ok(*self)
    }

    fn controller(&self) -> Result<&dyn Controller, WorldObjectError> {
        Err(Box::new(WandControllerError))
    }

    fn controller_mut(&mut self) -> Result<&mut dyn Controller, WorldObjectError> {
        Err(Box::new(WandControllerError))
    }

    fn take_controller(&mut self) -> Result<Box<dyn Controller>, WorldObjectError> {
        Err(Box::new(WandControllerError))
    }

    fn set_controller<C: Controller + 'static>(&mut self, controller: C) -> Result<(), (C, WorldObjectError)> {
        Err((controller, Box::new(WandControllerError)))
    }
}

#[derive(Debug)]
pub enum WandUseError {
    NoTargetProvided,
    FailedToGetUser(Box<dyn std::error::Error>),
    FailedToGetTarget(Box<dyn std::error::Error>),
    FailedToSetController(Box<dyn std::error::Error>),
    FailedToTakeController(Box<dyn std::error::Error>),
}

impl std::fmt::Display for WandUseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTargetProvided => write!(f, "wands must be used on a target"),
            Self::FailedToGetUser(err) => write!(f, "failed to get user: {}", err),
            Self::FailedToGetTarget(err) => write!(f, "failed to get target: {}", err),
            Self::FailedToSetController(err) => write!(f, "failed to set controller: {}", err),
            Self::FailedToTakeController(err) => write!(f, "failed to take controller: {}", err),
        }
    }
}

impl std::error::Error for WandUseError {}

impl InventoryItem for Wand {
    fn dummy(&self) -> Box<dyn InventoryItem> {
        Box::new(<Wand as TypedWorldObject>::dummy(self))
    }

    fn use_item(&mut self, world: &World, user_handle: WorldObjectHandle, target_handle: Option<WorldObjectHandle>) -> Result<Action, Box<dyn std::error::Error>> {
        let target_description = target_handle.as_ref().ok_or(WandUseError::NoTargetProvided)
            .and_then(|handle| world.get_object(&handle)
                .map_err(|err| WandUseError::FailedToGetTarget(Box::new(err))))
            .map(|object| object.definite_description())?;


        Ok(Action{
            exec: Box::new(move |world| {
                let user_handle = user_handle.clone();
                let target_handle = target_handle.clone();
                Box::pin(async move {
                    let target_handle = target_handle
                        .ok_or::<Box<dyn std::error::Error>>(Box::new(WandUseError::NoTargetProvided).into())?;

                    let user_controller = world.get_object_mut(&user_handle)
                        .map_err(|err| -> Box<dyn std::error::Error> { 
                            Box::new(WandUseError::FailedToGetUser(Box::new(err))).into()
                        })?.take_controller()?;

                    let target_controller = world.get_object_mut(&target_handle)
                        .map_err(|err| -> Box<dyn std::error::Error> {
                            Box::new(WandUseError::FailedToGetTarget(Box::new(err)))
                        })?.take_controller()?;

                    world.get_object_mut(&target_handle)
                        .map_err(|err| -> Box<dyn std::error::Error> {
                            Box::new(WandUseError::FailedToGetTarget(Box::new(err))).into()
                        })?.set_controller(user_controller).map_err(
                            |(_, err)| err
                        )?;
                    
                    world.get_object_mut(&user_handle)
                        .map_err(|err| -> Box<dyn std::error::Error> {
                            Box::new(WandUseError::FailedToGetUser(Box::new(err))).into()
                        })?.set_controller(target_controller).map_err(
                            |(_, err)| err
                        )?;

                    Ok(None)
                })
            }),
            verb_phrase: VerbPhrase::Prepositional(
                PrepositionalVerbPhrase {
                    main_verb_phrase: Box::new(
                        VerbPhrase::Transitive(
                            TransitiveVerbPhrase {
                                verb: TransitiveVerb::new(ToCast),
                                direct_object: String::from("transmogrify"),
                            }
                        )
                    ),
                    prepositional_phrase: PrepositionalPhrase {
                        preposition: String::from("on"),
                        object: target_description,
                    }
                }
            )
        })
    }
}