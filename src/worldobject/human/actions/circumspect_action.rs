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
                    let result: String = todo!("circumspect via world object listing");
                    Ok(Some(result))
                })
            }
        ),
        verb_phrase: VerbPhrase::Intransitive(
            IntransitiveVerb::new(ToCircumspect)
        )
    }
}