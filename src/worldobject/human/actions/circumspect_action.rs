use crate::{
    lang::{VerbPhrase, IntransitiveVerb, verbs::ToCircumspect},
    world::World,
    worldobject::fns::update::Action
};

pub fn action() -> Action {
    Action{
        exec: Box::new(
            move |world: &mut World| {
                Box::pin(async move {
                    let handles_and_descriptions = world.objects.iter()
                        .map(|(handle, object)| (handle, format!("{}: {}", handle, object.1.indefinite_description())))
                        .collect::<Vec<_>>();

                    Ok(Some(if handles_and_descriptions.is_empty() {
                        format!("you see nothing around you")
                    } else {
                        format!("you see: \n - {}", handles_and_descriptions.iter().map(|(_, description)| description.clone()).collect::<Vec<_>>().join("\n - "))
                    }))
                })
            }
        ),
        verb_phrase: VerbPhrase::Intransitive(
            IntransitiveVerb::new(ToCircumspect)
        )
    }
}