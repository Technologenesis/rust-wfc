use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use crate::worldobject::human;
use crate::worldobject::human::controllers::HumanController;

use tokio_util::io::SyncIoBridge;

use super::message::NetworkHumanControllerMessage;

pub struct NetworkHumanControllerClient {
    subcontroller: Box<dyn HumanController>
}

impl NetworkHumanControllerClient {
    pub async fn connect(ip_address: String, character: human::Human) -> Result<(), Box<dyn std::error::Error>> {
        // Normalize "localhost" to "127.0.0.1" for consistency
        let addr = if ip_address == "localhost" {
            "127.0.0.1"
        } else {
            &ip_address
        };
        
        let mut stream = TcpStream::connect((addr, 8080)).await?;

        let (unsouled, controller) = character.desouled();

        let json = serde_json::to_vec(&unsouled)?;

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
                .map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid JSON")))?;
        }
    }

    async fn handle_message(message: NetworkHumanControllerMessage, subcontroller: &mut Box<dyn HumanController>, tcp_stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        match message {
            NetworkHumanControllerMessage::PromptTurn => {
                let action = subcontroller.prompt_turn()
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