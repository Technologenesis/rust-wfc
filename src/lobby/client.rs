use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use crate::worldobject::human;

pub struct LobbyClient {}

impl LobbyClient {
    pub async fn connect(ip_address: String, character: human::UnsouledHuman) -> Result<Self, Box<dyn std::error::Error>> {
        // Normalize "localhost" to "127.0.0.1" for consistency
        let addr = if ip_address == "localhost" {
            "127.0.0.1"
        } else {
            &ip_address
        };
        
        let mut stream = TcpStream::connect((addr, 8080)).await?;

        let json = serde_json::to_vec(&character)?;

        //println!("Sending character to lobby: {}", String::from_utf8_lossy(&json));

        stream.write_all(&json).await?;

        stream.flush().await?;

        Ok(Self {})
    }
}