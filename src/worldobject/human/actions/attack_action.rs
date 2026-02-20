use crate::{
    lang::{VerbPhrase, TransitiveVerbPhrase, TransitiveVerb, verbs::ToAttack},
    worldobject::{
        fns::update::Action,
        human::Human,
        components::controllable::controller::commands::attack_command::AttackCommand
    },
    world::{World, WorldObjectGetError},
    quantities::direction::{DirectionHorizontal}
};

#[derive(Debug)]
pub enum AttackCommandToActionError {
    FailedToGetTargetObject(WorldObjectGetError),
}

impl std::fmt::Display for AttackCommandToActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToGetTargetObject(err) => write!(f, "failed to get target object: {}", err),
        }
    }
}

impl std::error::Error for AttackCommandToActionError {}

#[derive(Debug)]
pub enum AttackError {
    NoCapableBodyParts,
}

impl std::fmt::Display for AttackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCapableBodyParts => write!(f, "none of your body parts are capable of attacking"),
        }
    }
}

impl std::error::Error for AttackError {}

pub fn from_command(cmd: AttackCommand, world: &World, me: Human) -> Result<Action, AttackCommandToActionError> {
    let target_description = world.get_object(&cmd.target_handle)
        .map(|object| object.linguistics().definite_description.clone())
        .map_err(|err| AttackCommandToActionError::FailedToGetTargetObject(err))?;

    Ok(Action{
        exec: Box::new(
            move |world: &mut World| {
                Box::pin(async move {
                    let arm = match me.dominant_arm {
                        DirectionHorizontal::Left => &me.body.torso.left_arm,
                        DirectionHorizontal::Right => &me.body.torso.right_arm,
                    };
                
                    let punch_force = arm.punch_force.clone();
                    
                    let msg: String = todo!("apply_force via PhysicsObjectTrait");
                
                    Ok(Some(msg))
                })
            }
        ),
        verb_phrase: VerbPhrase::Transitive(
            TransitiveVerbPhrase {
                verb: TransitiveVerb::new(ToAttack),
                direct_object: target_description
            }
        )
    })
}

