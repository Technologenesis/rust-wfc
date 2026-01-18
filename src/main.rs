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

use {
    materials::Material,
    lobby::host,
    std::{collections::HashMap, io, fs::File},
    logging::{basic::BasicLogger, channel::LoggingChannel},
    world::{World, coord::WorldCoord},
    worldobject::{WorldObject, sword::Sword, rat::Rat,
        components::controllers::net::client::NetworkHumanControllerClient
    },
    quantities::{distance::meters, mass::kilograms, speed::meters_per_second}
};

#[tokio::main]
async fn main() {
    let character = character_creation::create_character();

    println!("HOST or JOIN?");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    choice = choice.trim().to_lowercase();

    let out = File::create("world.log").unwrap();

    let logger = BasicLogger::new(out)
        .format(String::from("{timestamp} [{level}]: {message}"), HashMap::new());
    // logging_channel takes ownership of the logger.
    // it can then be used to create new loggers which will forward messages to the original logger
    // in a thread-safe manner.
    let logging_channel = LoggingChannel::new(logger);

    if choice == "host" {
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

        // add local player
        world.add_object(
            character.name(),
            Box::new(character),
            WorldCoord::new(meters(0.0), meters(0.0))
        );

        host(logging_channel.logger(), Box::new(move || logging_channel.logger().to_dyn()), world).await.unwrap();
    } else if choice == "join" {
        println!("Enter the IP address of the lobby you want to join:");

        let mut ip_address = String::new();
        io::stdin().read_line(&mut ip_address).unwrap();
        ip_address = ip_address.trim().to_string();

        NetworkHumanControllerClient::connect(ip_address, character).await.unwrap();
    }
}