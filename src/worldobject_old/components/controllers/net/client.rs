use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use crate::worldobject::{
    human,
    WorldObject
};

use super::super::Controller;

use tokio_util::io::SyncIoBridge;

use super::message::NetworkHumanControllerMessage;

// NetworkHumanControllerClient wraps a HumanController
// and communicates with a remote NetworkHumanController,
// allowing the underlying controller to remotely control
// a human character over a network connection.
pub struct NetworkHumanControllerClient {
    subcontroller: Box<dyn Controller>
}

impl NetworkHumanControllerClient {
    // connect connects to a remote NetworkHumanController
    // at the given IP address and port,
    // and registers the given character with the remote controller.
    // It then perpetually waits for prompts from the remote controller,
    // forwards them to the underlying local controller, and sends the results
    // back to the remote controller.
    pub async fn connect(ip_address: String, mut character: human::Human) -> Result<(), Box<dyn std::error::Error>> {
        // Normalize "localhost" to "127.0.0.1" for consistency
        let addr = if ip_address == "localhost" {
            "127.0.0.1"
        } else {
            &ip_address
        };
        
        let mut stream = TcpStream::connect((addr, 25565)).await?;

        let controller = character.take_controller()?;

        let json = serde_json::to_vec(&character)?;

        //println!("Sending character to lobby: {}", String::from_utf8_lossy(&json));

        stream.write_all(&json).await?;

        stream.flush().await?;

        let mut client = Self{
            subcontroller: controller,
        };

        println!("Connected to lobby!  Waiting for your turn...");

        loop {
            let message = tokio::task::block_in_place(
                ||
                    serde_json::Deserializer::from_reader(SyncIoBridge::new(&mut stream)).into_iter::<serde_json::Value>().next().
                    transpose().
                    map_err(
                        |e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                    )?.
                    ok_or(
                        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid JSON"))
                    ).
                    and_then(
                        |message| serde_json::from_value::<NetworkHumanControllerMessage>(message).map_err(
                            |e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                        )
                    )
            )?;

            tokio::task::block_in_place(|| Self::handle_message(message, &mut client.subcontroller, &mut stream)).await
                .map_err(|err| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string())))?;
        }
    }

    // handle_message handles a message from the remote controller
    // by forwarding it to the underlying local controller
    // and sending the results back to the remote controller.
    async fn handle_message(message: NetworkHumanControllerMessage, subcontroller: &mut Box<dyn Controller>, tcp_stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        match message {
            NetworkHumanControllerMessage::PromptTurn => {
                let action= subcontroller.prompt_turn()
                    .await?;
                let json = serde_json::to_vec(&action)?;
    
                //println!("Sending character to lobby: {}", String::from_utf8_lossy(&json));
    
                tcp_stream.write_all(&json).await.map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
    
                tcp_stream.flush().await?;

                Ok(())
            }
            NetworkHumanControllerMessage::DisplayMessage(message) => {
                subcontroller.display_message(message).await
            }
        }
    }
}