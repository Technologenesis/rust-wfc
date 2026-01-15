// This module houses the NetworkHumanController,
// which implements the HumanController trait
// and is used to communicate with a NetworkHumanControllerClient
// allowing a human character to be controlled over a network connection.

use crate::worldobject::human::{
    controllers,
    actions,
};

use async_trait::async_trait;

use crate::logging::Logger;
use crate::logging::LoggerImpl;

use super::message::NetworkHumanControllerMessage;

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio_util::io::SyncIoBridge;

// NetworkHumanController is a struct that implements the HumanController trait
// and is used to communicate with a NetworkHumanControllerClient
// allowing a human character to be controlled over a network connection.
pub struct NetworkHumanController {
    logger: Logger<Box<dyn LoggerImpl>>,
    tcp_stream: TcpStream
}

impl NetworkHumanController {
    // new creates a new NetworkHumanController from a TCP stream and a logger
    pub fn new(tcp_stream: TcpStream, logger: Logger<impl LoggerImpl + 'static>) -> Self {
        Self {
            tcp_stream,
            logger: logger.to_dyn(),
        }
    }
}

#[async_trait]
// NetworkHumanController implements the HumanController trait
impl controllers::HumanController for NetworkHumanController {
    // prompt_turn sends a prompt via the TCP stream
    // and waits for the client to send an action
    async fn prompt_turn(&mut self) -> Result<actions::HumanAction, Box<dyn std::error::Error>> {
        self.logger.info(String::from("prompting for turn across network controller...")).await;
        let json_content = serde_json::to_vec(&NetworkHumanControllerMessage::PromptTurn).map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        self.tcp_stream.write_all(&json_content).await.map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        self.logger.info(String::from("sent prompt; waiting for action from network controller...")).await;
        
        self.logger.info(String::from("reading action from network controller...")).await;
        let mut json_stream = serde_json::Deserializer::from_reader(SyncIoBridge::new(&mut self.tcp_stream)).into_iter::<serde_json::Value>();
        let next_json = tokio::task::block_in_place(|| {
            json_stream.next().ok_or(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid JSON")))
        })?;
        self.logger.info(String::from("received action from network controller...")).await;
    
        match next_json {
            Ok(json) => {
                self.logger.info(format!("Received action: {}", json.to_string())).await;
                if let Ok(action) = serde_json::from_value(json) {
                    Ok(action)
                }
                else {
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid JSON")))
                }
            }
            Err(e) => {
                Err(Box::new(e))
            }
        }
    }

    // display_message sends a message via the TCP stream to be displayed to the remote user
    async fn display_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        let json_content = serde_json::to_vec(&NetworkHumanControllerMessage::DisplayMessage(message)).map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        self.tcp_stream.write_all(&json_content).await.map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        self.tcp_stream.flush().await.map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
        Ok(())
    }
}