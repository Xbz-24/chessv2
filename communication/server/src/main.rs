mod connection;
mod game_session;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize server here
    // For example, start listening on a certain port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            // This connection will be handled in connection.rs
            if let Err(e) = connection::handle_connection(socket).await {
                eprintln!("An error occurred while handling a connection: {}", e);
            }
        });
    }
}
