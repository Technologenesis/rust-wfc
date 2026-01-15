use crate::worldobject::human::{
    controllers,
    actions,
};

use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;

use crate::logging::Logger;
use crate::logging::LoggerImpl;

use super::message::NetworkHumanControllerMessage;

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio_util::io::SyncIoBridge;
use serde::Serializer;

pub struct NetworkHumanController {
    logger: Logger<Box<dyn LoggerImpl>>,
    tcp_stream: TcpStream
}

impl NetworkHumanController {
    pub fn new(tcp_stream: TcpStream, logger: Logger<impl LoggerImpl + 'static>) -> Self {
        Self {
            tcp_stream,
            logger: logger.to_dyn(),
        }
    }
}

#[async_trait]
impl controllers::HumanController for NetworkHumanController {
    async fn prompt_turn(&mut self) -> Result<actions::HumanAction, Box<dyn std::error::Error>> {
        let json_content = serde_json::to_vec(&NetworkHumanControllerMessage::PromptTurn).map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        self.tcp_stream.write_all(&json_content).await.map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        
        let mut json_stream = serde_json::Deserializer::from_reader(SyncIoBridge::new(&mut self.tcp_stream)).into_iter::<serde_json::Value>();
        
        let next_json = tokio::task::block_in_place(|| {
            json_stream.next().ok_or(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid JSON")))
        })?;
    
        match next_json {
            Ok(json) => {
                println!("Received action: {}", json.to_string());
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

    async fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        let json_content = serde_json::to_vec(&NetworkHumanControllerMessage::DisplayMessage(message)).map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        self.tcp_stream.write_all(&json_content).await.map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        self.tcp_stream.flush().await.map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        Ok(())
    }
}