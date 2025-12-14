#![feature(never_type)]

mod worldobject;
mod world;
mod human;
mod rat;
mod sword;
mod materials;
mod components;
mod quantities;
mod lobby;

use std::io;

use human::controllers::terminal;
use human::body::arm;
use human::body::arm::hand;
use components::inventory;

use quantities::distance;
use quantities::force;
use quantities::duration;
use quantities::mass;
use quantities::speed;

use crate::human::gender;
use crate::quantities::direction;

#[tokio::main]
async fn main() {
    println!("You awaken - your vision is filled with darkness.  Your memory is hazy.  While your eyes adjust, you talk yourself through some basic facts...");

    let character = create_character();

    println!("HOST or JOIN?");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    choice = choice.trim().to_lowercase();
    if choice == "host" {
        let characters = start_lobby(human::Human::new(character, human::controllers::terminal::TerminalHumanController{})).await.unwrap();
        println!("Characters: {:?}", characters.iter().map(|c| format!("{} ({})", c.name(), c.examine())).collect::<Vec<String>>());
    } else if choice == "join" {
        println!("Enter the IP address of the lobby you want to join:");

        let mut ip_address = String::new();
        io::stdin().read_line(&mut ip_address).unwrap();
        ip_address = ip_address.trim().to_string();

        let client = lobby::client::LobbyClient::connect(ip_address, character).await.unwrap();
    }
}

fn create_character() -> human::UnsouledHuman {
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
    let mut gender_res = gender::Gender::try_from(gender_str.as_str());
    while !gender_res.is_ok() {
        gender_str = String::new();

        res = io::stdin().read_line(&mut gender_str);
        while !res.is_ok() {
            println!("Your brain is still muddled; you try again.  What is your gender?");
            println!("(MALE, FEMALE, or OTHER)?");
            res = io::stdin().read_line(&mut gender_str);
        }

        gender_res = gender::Gender::try_from(gender_str.as_str());
    }
    let gender = gender_res.unwrap();
    println!("After some effort, you bring forth clear memories of being a {}.", gender.noun());

    human::UnsouledHuman::new(
        name,
        gender,
        speed::meters_per_second(5.0),
        Some(arm::arm(
            mass::kilograms(10.0),
            distance::meters(1.0),
            force::newtons(1000.0),
            Some(hand::hand(None::<Box<dyn inventory::InventoryItem>>)),
        )),
        Some(arm::arm(
            mass::kilograms(10.0),
            distance::meters(1.0),
            force::newtons(1000.0),
            Some(hand::hand(None::<Box<dyn inventory::InventoryItem>>)),
        )),
        direction::DirectionHorizontal::Right,
        mass::kilograms(0.0),
        inventory::Inventory::new(),
    )
}

async fn start_lobby<'a>(character: human::Human) -> Result<Vec<Box<dyn worldobject::WorldObject>>, ()> {
    let mut lobby = lobby::Lobby::new();

    lobby.add_character(character)
        .map_err(|_| ())?;

    let characters = lobby.open().await.unwrap();

    Ok(characters)
}