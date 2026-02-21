use crate::{
    worldobject::{fns::update::Action, human::Human,
        components::controllable::controller::commands::use_command::UseCommand
    },
    world::{World, handle::WorldObjectHandle},
    quantities::direction::DirectionHorizontal,
};

#[derive(Debug)]
pub enum UseCommandToActionError {
    NoHand,
    NotWieldingAnything,
    FailedToFindWieldedItem(String),
    ItemNotUsable(String),
    FailedToUseItem(Box<dyn std::error::Error>),
}

impl std::fmt::Display for UseCommandToActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoHand => write!(f, "you don't have a hand to use items with"),
            Self::NotWieldingAnything => write!(f, "you are not wielding anything"),
            Self::FailedToFindWieldedItem(name) => write!(f, "you are not wielding an item named \"{}\"", name),
            Self::ItemNotUsable(name) => write!(f, "the {} cannot be used", name),
            Self::FailedToUseItem(err) => write!(f, "failed to use item: {}", err),
        }
    }
}

impl std::error::Error for UseCommandToActionError {}

pub fn from_command(me: &mut Human, _my_handle: WorldObjectHandle, cmd: UseCommand, _world: &World) -> Result<Action, UseCommandToActionError> {
    let arm = match me.dominant_arm {
        DirectionHorizontal::Left => &mut me.body.torso.left_arm,
        DirectionHorizontal::Right => &mut me.body.torso.right_arm,
    };

    let hand = arm.hand.as_mut()
        .ok_or(UseCommandToActionError::NoHand)?;

    let item = hand.held_item.as_mut()
        .ok_or(UseCommandToActionError::NotWieldingAnything)?;

    if item.wieldable_name() != cmd.item_name {
        return Err(UseCommandToActionError::FailedToFindWieldedItem(cmd.item_name));
    }

    let usable = item.as_usable()
        .ok_or_else(|| UseCommandToActionError::ItemNotUsable(cmd.item_name.clone()))?;

    let effect = usable.use_item(cmd.target_handle.as_ref())
        .map_err(UseCommandToActionError::FailedToUseItem)?;

    let message = effect.message;
    let verb_phrase = effect.verb_phrase;

    Ok(Action {
        exec: Box::new(move |_: &mut World| {
            Box::pin(async move {
                Ok(Some(message))
            })
        }),
        verb_phrase,
    })
}

