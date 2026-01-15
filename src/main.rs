#![feature(never_type)]
#![feature(async_trait_bounds)]

mod worldobject;
mod world;
mod materials;
mod quantities;
mod logging;

use std::{
    collections::HashMap,
    io,
    fs::File,
};

use chrono::Utc;

use logging::{
    Logger,
    LoggerImpl,
    basic::BasicLogger,
    channel::LoggingChannel,
};

use world::{
    World,
    coord::WorldCoord,
};

use materials::Material;

use worldobject::{
    WorldObject,
    sword::Sword,
    rat::Rat,
    human::{
        Human,
        controllers::{
            net::{
                Lobby,
                client::NetworkHumanControllerClient
            },
            terminal::TerminalHumanController
        },
        unsouled::{
            UnsouledHuman,
            body::{
                arm::arm,
                arm::hand::hand,
            },
            gender::Gender
        }
    },
    components::inventory::{
        Inventory,
        item::InventoryItem
    }
};

use quantities::{
    distance::meters,
    mass::kilograms,
    speed::meters_per_second,
    force::newtons,
    direction::DirectionHorizontal
};

#[tokio::main]
async fn main() {
    println!("You awaken - your vision is filled with darkness.  Your memory is hazy.  While your eyes adjust, you talk yourself through some basic facts...");

    let character = create_character();

    println!("HOST or JOIN?");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    choice = choice.trim().to_lowercase();

    let out = File::create("world.log").unwrap();

    let mut kwargs = HashMap::new();
    kwargs.insert(String::from("timestamp"), String::from(Utc::now().to_string()));

    let logger = BasicLogger::new(out)
        .format(String::from("{timestamp} [{level}]: {message}"), kwargs);
    // logging_channel takes ownership of the logger.
    // it can then be used to create new loggers which will forward messages to the original logger
    // in a thread-safe manner.
    let logging_channel = LoggingChannel::new(logger);

    if choice == "host" {
        let characters = {
            // clone the logging channel so that the lobby can use it to
            // create a new logger for each network controller.
            let logging_channel = logging_channel.clone();
            start_lobby(
                logging_channel.logger().to_dyn(),
                Box::new(move || logging_channel.logger().to_dyn()),
                Human::new(
                    character,
                    TerminalHumanController{}
                )
            ).await.unwrap()
        };
        println!("Characters: {:?}", characters.iter().map(|c| format!("{} ({})", c.name(), c.examine())).collect::<Vec<String>>());

        let mut world = World::new(logging_channel.logger());

        // populate non-character objects
        world.add_object(
            String::from("sword"),
            Box::new(Sword::new(
                meters(1.0),
                Material::Iron
            )),
            WorldCoord::new(meters(1.0), meters(0.0))
        );
        world.add_object(
            String::from("rat"),
            Box::new(Rat::new(
                kilograms(1.0),
                meters_per_second(1.0)
            )),
            WorldCoord::new(meters(2.0), meters(0.0))
        );

        for character in characters {
            println!("Adding character to world: {}", character.name());
            world.add_object(character.name(), character, WorldCoord::new(quantities::distance::meters(0.0), quantities::distance::meters(0.0)));
        }

        loop {
            match world.update().await {
                Ok(()) => (),
                Err(err) => {
                    println!("Error updating world: {:?}", err);
                }
            }
        }
    } else if choice == "join" {
        println!("Enter the IP address of the lobby you want to join:");

        let mut ip_address = String::new();
        io::stdin().read_line(&mut ip_address).unwrap();
        ip_address = ip_address.trim().to_string();

        NetworkHumanControllerClient::connect(ip_address, Human::new(character, TerminalHumanController{})).await.unwrap();
    }
}

fn create_character() -> UnsouledHuman {
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

    UnsouledHuman::new(
        name,
        gender,
        meters_per_second(5.0),
        Some(arm(
            kilograms(10.0),
            meters(1.0),
            newtons(1000.0),
            Some(hand(
                kilograms(1.0),
                None::<Box<dyn InventoryItem>>)),
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
    )
}

async fn start_lobby<'a>(logger: Logger<impl LoggerImpl + 'static>, new_controller_logger: Box<dyn Fn() -> Logger<Box<dyn LoggerImpl>>>, character: Human) -> Result<Vec<Box<dyn WorldObject>>, ()> {
    let mut lobby = Lobby::new(logger, new_controller_logger);

    lobby.add_character(character)
        .map_err(|_| ())?;

    let characters = lobby.open().await.unwrap();

    Ok(characters)
}