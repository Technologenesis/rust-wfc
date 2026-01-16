use std::io;

use crate::{
    worldobject::{
        components::inventory::{
            Inventory,
            item::InventoryItem
        },
        human::{
            Human,
            controllers::terminal::TerminalHumanController,
            unsouled::{
                UnsouledHuman,
                gender::Gender,
                body::arm::{
                    arm,
                    hand::hand
                }
            },
        }
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
        UnsouledHuman::new(
            name,
            gender,
            meters_per_second(5.0),
            Some(arm(
                kilograms(10.0),
                meters(1.0),
                newtons(1000.0),
                Some(
                    hand(
                        kilograms(1.0),
                        None::<Box<dyn InventoryItem>>
                    )
                )
            )),
            Some(arm(
                kilograms(10.0),
                meters(1.0),
                newtons(1000.0),
                Some(hand(
                    kilograms(1.0),
                    None::<Box<dyn InventoryItem>>
                )),
            )),
            DirectionHorizontal::Right,
            kilograms(0.0),
            Inventory::new(),
        ),
        TerminalHumanController{}
    )
}
