use crate::{
    lang::{VerbPhrase, TransitiveVerb, TransitiveVerbPhrase, verbs::ToCheck},
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
                    Ok(Some(format!(
                        "{}\n{}\n{}",
                        {
                            let handles_and_descriptions = me.inventory()?.0.iter()
                                .map(|(handle, object)| (handle, format!("{}: {}", handle, object.definite_description())))
                                .collect::<Vec<_>>();

                            if handles_and_descriptions.is_empty() {
                                format!("you have are carrying nothing")
                            } else {
                                format!("you are carrying:\n - {}", handles_and_descriptions.iter().map(|(_, description)| description.clone()).collect::<Vec<_>>().join("\n - "))
                            }
                        },
                        format!("in your left hand, you are wielding {}", me.body.torso.left_arm.wielded_item()
                            .map(|item| item.indefinite_description())
                            .unwrap_or(String::from("nothing"))
                        ),
                        format!("in your right hand, you are wielding {}", me.body.torso.right_arm.wielded_item()
                            .map(|item| item.indefinite_description())
                            .unwrap_or(String::from("nothing"))
                        ),
                    )))
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