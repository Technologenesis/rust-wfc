use crate::worldobject::human::{
    controllers,
    actions,
};
use crate::world;

use tokio::net::TcpStream;
use tokio_util::io::SyncIoBridge;
use serde::Serializer;

pub struct NetworkHumanController {
    tcp_stream: TcpStream
}

impl NetworkHumanController {
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self { tcp_stream }
    }
}

impl controllers::HumanController for NetworkHumanController {
    fn prompt_turn(&mut self, _: &world::World) -> Result<actions::HumanAction, Box<dyn std::error::Error>> {
        let mut json_stream = serde_json::Deserializer::from_reader(SyncIoBridge::new(&mut self.tcp_stream)).into_iter::<serde_json::Value>();

        let next_json = tokio::task::block_in_place(|| {
            json_stream.next()
        }).unwrap();

        match next_json {
            Ok(json) => {
                if let Ok(action) = serde_json::from_value(json) {
                    Ok(action)
                }
                else {
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid JSON")))
                }
            }
            Err(e) => {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))
            }
        }
    }

    fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut json_stream = serde_json::Serializer::new(SyncIoBridge::new(&mut self.tcp_stream));
        json_stream.serialize_str(&message)?;
        Ok(())
    }
}