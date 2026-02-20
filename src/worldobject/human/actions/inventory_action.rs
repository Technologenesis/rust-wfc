use crate::{
    lang::{VerbPhrase, TransitiveVerb, TransitiveVerbPhrase, verbs::ToCheck},
    world::World,
    worldobject::{
        WorldObject,
        fns::update::Action,
        human::Human
    }
};

pub fn action(me: Human) -> Action {
    Action{
        exec: Box::new(
            move |_: &mut World| {
                Box::pin(async move {
                    let result: String = todo!("inventory display with new component API");
                    Ok(Some(result))
                })
            }
        ),
        verb_phrase: VerbPhrase::Transitive(
            TransitiveVerbPhrase {
                verb: TransitiveVerb::new(ToCheck),
                direct_object: String::from("inventory")
            }
        )
    }
}