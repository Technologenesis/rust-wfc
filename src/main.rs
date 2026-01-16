#![feature(never_type)]
#![feature(async_trait_bounds)]

mod worldobject;
mod world;
mod materials;
mod quantities;
mod logging;
mod lang;
mod util;
mod character_creation;
mod lobby;

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

use lobby::Lobby;

use worldobject::{
    WorldObject,
    sword::Sword,
    rat::Rat,
    human::{
        Human,
        controllers::{
            net::client::NetworkHumanControllerClient,
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
    let character = character_creation::create_character();

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

async fn start_lobby<'a>(logger: Logger<impl LoggerImpl + 'static>, new_controller_logger: Box<dyn Fn() -> Logger<Box<dyn LoggerImpl>>>, character: Human) -> Result<Vec<Box<dyn WorldObject>>, ()> {
    let mut lobby = Lobby::new(logger, new_controller_logger);

    lobby.add_character(character)
        .map_err(|_| ())?;

    let characters = lobby.open().await.unwrap();

    Ok(characters)
}