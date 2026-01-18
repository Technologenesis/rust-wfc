use crate::{
    lang::{VerbPhrase, IntransitiveVerb, verbs::ToCheck},
    world::World,
    worldobject::{
        TypedWorldObject,
        fns::update::Action,
        human::unsouled::UnsouledHuman
    }
};

pub fn action(me: UnsouledHuman) -> Action {
    Action{
        exec: Box::new(
            move |_: &mut World| {
                Box::pin(async move {
                    let handles_and_descriptions = me.inventory()?.0.iter()
                        .map(|(handle, object)| (handle, format!("{}: {}", handle, object.definite_description())))
                        .collect::<Vec<_>>();

                    Ok(Some(if handles_and_descriptions.is_empty() {
                        format!("you have are carrying nothing")
                    } else {
                        format!("you are carrying:\n - {}", handles_and_descriptions.iter().map(|(_, description)| description.clone()).collect::<Vec<_>>().join("\n - "))
                    }))
                })
            }
        ),
        verb_phrase: VerbPhrase::Intransitive(
            IntransitiveVerb::new(ToCheck)
        )
    }
}