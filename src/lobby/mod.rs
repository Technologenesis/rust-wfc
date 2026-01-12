pub mod client;

use std::sync::Mutex;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use std::net::SocketAddr;
use tokio_util::io::SyncIoBridge;
use std::io;

use crate::worldobject::human;
use crate::worldobject;

pub struct Lobby {
    characters: Vec<Box<dyn worldobject::WorldObject>>,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            characters: Vec::new(),
        }
    }

    pub fn add_character<'a>(
        &'a mut self,
        character: impl worldobject::WorldObject + 'static,
    ) -> Result<(), Box<dyn std::error::Error + 'a>> {
        let name = character.name();

        self.characters.push(Box::new(character));

        println!("\"{}\" has joined the lobby", name);

        Ok(())
    }

    // create a new open lobby
    pub async fn open(mut self) -> Result<Vec<Box<dyn worldobject::WorldObject>>, ()> {
        let listener = TcpListener::bind(("127.0.0.1", 8080))
            .await
            .map_err(|_| ())?;

        println!("Listening for connections; press enter to close the lobby and start your journey...");
        loop {
            tokio::select! {
                stream_and_socket_addr_result = listener.accept() => match stream_and_socket_addr_result {
                    Ok((stream, socket_addr)) => {
                        println!("Received connection from {}", socket_addr);
                        self.register_connection_with_lobby(stream, socket_addr).await;
                    },
                    Err(_) => ()
                },
                _ = wait_for_line() => {
                    println!("Received stop signal");
                    break;
                }
            };
        };

        Ok(self.characters)
    }

    async fn register_connection_with_lobby(&mut self, mut stream: TcpStream, _:  SocketAddr) {
        println!("Reading character information from connection...");
        let mut json_stream = serde_json::Deserializer::from_reader(SyncIoBridge::new(&mut stream)).into_iter::<serde_json::Value>();

        let next_json = tokio::task::block_in_place(|| {
            json_stream.next()
        }).unwrap().unwrap();

        //println!("Received character information: {}", (&next_json).to_string());

        if let Ok(unsouled) = human::UnsouledHuman::try_from(&next_json) {
            // TODO: Create proper network controller
            if let Err(err) = self.add_character(human::Human::new(unsouled, human::controllers::network::NetworkHumanController::new(stream))) {
                println!("Error adding character: {}", err);
            }
        } else if let Err(err) = human::UnsouledHuman::try_from(&next_json) {
            println!("Error parsing character information: {}", err);
        }
    }
}

async fn wait_for_line() {
    tokio::task::spawn_blocking(|| {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
    }).await.unwrap();
}