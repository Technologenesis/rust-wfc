pub mod client;

use std::sync::Mutex;

use tokio::net::TcpListener;
use tokio::sync::mpsc as mpsc_tokio;
use std::sync::mpsc;
use tokio::net::TcpStream;
use std::net::SocketAddr;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use tokio_util::io::SyncIoBridge;
use std::io;

use crate::human;
use crate::quantities::distance;
use crate::world;
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
        let mut json_stream = serde_json::Deserializer::from_reader(SyncIoBridge::new(&mut stream)).into_iter::<serde_json::Value>();

        let next_json = tokio::task::block_in_place(|| {
            json_stream.next()
        }).unwrap();

        match next_json {
            Ok(json) => {
                if let Ok(unsouled) = serde_json::from_value(json) {
                    // TODO: Create proper network controller
                    self.add_character(human::Human::new(unsouled, human::controllers::network::NetworkHumanController{stream}));
                }
            }
            Err(e) => {
                println!("Error parsing JSON: {}", e);
            }
        }
    }
}

async fn wait_for_line() {
    tokio::task::spawn_blocking(|| {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
    }).await.unwrap();
}