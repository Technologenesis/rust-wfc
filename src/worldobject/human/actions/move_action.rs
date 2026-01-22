use futures::future::BoxFuture;

use crate::{
    lang::{VerbPhrase, IntransitiveVerb, verbs::ToMove},
    world::{World, handle::WorldObjectHandle},
    worldobject::{
        Error as WorldObjectError,
        components::controllers::commands::move_command::MoveCommand,
        human::Human,
        fns::update::Action
    },
    quantities::{
        duration::seconds,
        distance::meters
    }
};

#[derive(Debug)]
pub enum MoveError {
    DistanceTooGreat,
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DistanceTooGreat => write!(f, "distance too great"),
        }
    }
}

impl std::error::Error for MoveError {}

pub fn from_command(cmd: MoveCommand, my_handle: WorldObjectHandle, me: Human) -> Action {
    Action{
        exec: Box::new(
            move |world: &mut World| -> BoxFuture<Result<Option<String>, WorldObjectError>> {
                Box::pin(async move {
                    if cmd.distance > (seconds(1.0) * me.body.legs.speed.commute()).associate_left().commute().cancel() {
                        return Err(Box::new(MoveError::DistanceTooGreat).into());
                    }

                    world.move_object(&my_handle, &cmd.direction, &cmd.distance)
                        .map_err(|err| Box::new(err))?;

                    let dist_f64 = (&cmd.distance / &meters(1.0)).cancel().0.0;

                    Ok(Some(format!("you move {} meters {}", dist_f64, cmd.direction)))
                })
            }
        ),
        verb_phrase: VerbPhrase::Intransitive(
            IntransitiveVerb::new(ToMove)
        )
    }
}