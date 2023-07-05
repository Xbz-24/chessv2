use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use std::error::Error;
pub async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    // Here you can handle the incoming connection
    // You could read messages from the client and handle them accordingly
    let mut buffer = [0; 1024];
    loop {
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        // Handle the received message here
        // If a new game session is started, it should be handled in game_session.rs
        // game_session::start_new_game();
    }
    Ok(())
}
