
mod connection;
mod server_handler;
mod game_interface;
mod game_logic_proxy;

use tokio::sync::Mutex;
use std::sync::Arc;
use crate::game_interface::GameInterface;
use crate::connection::ServerConnection;

fn main() {
    // Create a new connection to the server
    let server_connection_result = connection::ServerConnection::new("127.0.0.1:8080");

    // Handle the result of creating the server connection
    match server_connection_result {
        Ok(server_connection) => {
            // Start the game interface
            let game_interface = game_interface::GameInterface::new();
            game_interface.start(server_connection);
        }
        Err(error) => {
            eprintln!("Failed to create server connection: {}", error);
            // Handle the error gracefully, e.g., by displaying an error message or exiting the program
        }
    }
}

