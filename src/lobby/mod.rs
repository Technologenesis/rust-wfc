use std::{
    net::SocketAddr,
    io,
};

use tokio::{
    net::{
        TcpListener,
        TcpStream
    }
};

use tokio_util::io::SyncIoBridge;

use crate::{
    logging::{
        Logger,
        LoggerImpl,
    },
    worldobject::{
        components::controllers::net::controller::NetworkController,
        WorldObject,
        TypedWorldObject,
        human::{
            Human
        }
    },
    world::{
        World,
        coord::WorldCoord
    },
    quantities::distance::meters
};

#[derive(Debug)]
pub enum HostError {
    AddCharacterError(Box<dyn std::error::Error>),
    ListenerError(Box<dyn std::error::Error>),
    RegisterConnectionError(Box<dyn std::error::Error>),
}

impl std::fmt::Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddCharacterError(e) => write!(f, "Error adding character: {}", e),
            Self::ListenerError(e) => write!(f, "Error creating listener: {}", e),
            Self::RegisterConnectionError(e) => write!(f, "Error registering connection: {}", e),
        }
    }
}

impl std::error::Error for HostError {}

pub async fn host(
    logger: Logger<impl LoggerImpl + 'static>,
    new_controller_logger: Box<dyn Fn() -> Logger<Box<dyn LoggerImpl>>>,
    mut world: World,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut lobby = Lobby::new(logger, new_controller_logger);

    let listener = TcpListener::bind(("0.0.0.0", 25565))
        .await
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    println!("Listening for connections; press enter to close the lobby and start your journey...");
    loop {
        // rather insanely, the only reason this entire project uses
        // tokio is to enable this specific piece of code.
        tokio::select! {
            stream_and_socket_addr_result = listener.accept() => match stream_and_socket_addr_result {
                Ok((stream, socket_addr)) => {
                    lobby.logger.info(format!("Received connection from {}", socket_addr)).await;
                    lobby.register_connection(stream, socket_addr).await;
                },
                Err(e) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))),
            },
            _ = wait_for_line() => {
                lobby.logger.info(String::from("Received stop signal")).await;
                break;
            }
        };
    };

    for character in lobby.characters {
        println!("Adding character to world: {}", character.name());
        world.add_object(character.name(), character, WorldCoord::new(meters(0.0), meters(0.0)));
    }
    println!("\n");

    loop {
        match world.update().await {
            Ok(()) => (),
            Err(err) => {
                println!("Error updating world: {:?}", err);
            }
        }
    }
}

pub struct Lobby {
    logger: Logger<Box<dyn LoggerImpl>>,
    new_controller_logger: Box<dyn Fn() -> Logger<Box<dyn LoggerImpl>>>,
    characters: Vec<Box<dyn WorldObject>>,
}

#[derive(Debug)]
pub enum LobbyError {
    HumanDeserializeError(String)
}

impl std::fmt::Display for LobbyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HumanDeserializeError(error) => write!(f, "Error deserializing human: {}", error),
        }
    }
}

impl std::error::Error for LobbyError {}

impl Lobby {
    fn new(logger: Logger<impl LoggerImpl + 'static>, new_controller_logger: Box<dyn Fn() -> Logger<Box<dyn LoggerImpl>>>) -> Self {
        Self {
            logger: logger.to_dyn(),
            new_controller_logger,
            characters: Vec::new(),
        }
    }

    fn add_character<'a>(
        &mut self,
        character: impl WorldObject + 'static,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let name = character.name();

        self.characters.push(Box::new(character));

        println!("\"{}\" has joined the lobby", name);

        Ok(())
    }

    async fn register_connection(&mut self, mut stream: TcpStream, _:  SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        println!("Reading character information from connection...");
        let mut json_stream = serde_json::Deserializer::from_reader(SyncIoBridge::new(&mut stream)).into_iter::<serde_json::Value>();

        let next_json = tokio::task::block_in_place(|| {
            json_stream.next()
        }).unwrap().unwrap();

        //println!("Received character information: {}", (&next_json).to_string());

        Human::try_from(&next_json)
            .map_err(|error| -> Box<dyn std::error::Error> { Box::new(LobbyError::HumanDeserializeError(error)) })
            .and_then(
                |mut character| {
                    <Human as TypedWorldObject>::set_controller(
                        &mut character,
                        NetworkController::new(stream, (self.new_controller_logger)())
                    )
                        .map_err(|(_, error)| error)?;

                    self.add_character(character)
                }
            )
    }
}

async fn wait_for_line() {
    tokio::task::spawn_blocking(|| {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
    }).await.unwrap();
}