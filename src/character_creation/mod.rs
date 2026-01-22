use std::io;

use crate::{
    worldobject::{
        components::{
            gender::Gender,
            body::{
                Body,
                head::{Head, Mouth},
                torso::{Torso, arm::arm, arm::hand::hand},
                legs::Legs
            },
            inventory::{
                Inventory,
                item::InventoryItem
            },
            controllers::terminal::TerminalHumanController,
        },
        human::Human
    },
    quantities::{
        speed::meters_per_second,
        mass::kilograms,
        distance::meters,
        force::newtons,
        direction::DirectionHorizontal,
    }
};

pub fn create_character() -> Human {
    println!("You awaken - your vision is filled with darkness.  Your memory is hazy.  While your eyes adjust, you talk yourself through some basic facts...");

    println!("What is your name?");
    let mut name = String::new();

    let res = io::stdin().read_line(&mut name);
    while !res.is_ok() {
        println!("Your brain is still muddled; you try again.  What is your name?");
    }
    name = name.trim().to_string();
    println!("Yes, yes, {}... that was it.  You continue...", name);

    println!("What is your gender?");
    println!("(MALE, FEMALE, or OTHER)?");
    let mut gender_str = String::new();

    let mut res = io::stdin().read_line(&mut gender_str);
    while !res.is_ok() {
        println!("Your brain is still muddled; you try again.  What is your gender?");
        println!("(MALE, FEMALE, or OTHER)?");
        res = io::stdin().read_line(&mut gender_str);
    }
    gender_str = gender_str.trim().to_string();
    let mut gender_res = Gender::try_from(gender_str.as_str());
    while !gender_res.is_ok() {
        gender_str = String::new();

        res = io::stdin().read_line(&mut gender_str);
        while !res.is_ok() {
            println!("Your brain is still muddled; you try again.  What is your gender?");
            println!("(MALE, FEMALE, or OTHER)?");
            res = io::stdin().read_line(&mut gender_str);
        }

        gender_res = Gender::try_from(gender_str.as_str());
    }
    let gender = gender_res.unwrap();
    println!("After some effort, you bring forth clear memories of being a {}.", gender.noun());

    Human::new(
        name,
        gender,
        Body{
            base_mass: kilograms(100.0),
            head: Head{
                base_mass: kilograms(7.0),
                mouth: Mouth{
                    base_mass: kilograms(1.0),
                    teeth: vec![],
                },
            },
            torso: Torso{
                base_mass: kilograms(20.0),
                left_arm: arm(
                    kilograms(10.0),
                    meters(1.0),
                    newtons(1000.0),
                    Some(hand(
                        kilograms(1.0),
                        None::<Box<dyn InventoryItem>>
                    ))
                ),
                right_arm: arm(
                    kilograms(10.0),
                    meters(1.0),
                    newtons(1000.0),
                    Some(hand(
                        kilograms(1.0),
                        None::<Box<dyn InventoryItem>>
                    ))
                ),
            },
            legs: Legs{
                base_mass: kilograms(10.0),
                speed: meters_per_second(5.0),
            },
        },
        DirectionHorizontal::Right,
        Inventory::new(),
        Some(TerminalHumanController{})
    )
}
