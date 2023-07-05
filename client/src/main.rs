// File: main.rs
mod connection;
mod server_handler;
mod game_interface;
mod game_logic_proxy;

use tokio::sync::Mutex;
use std::sync::Arc;
use crate::game_interface::GameInterface;
use crate::connection::ServerConnection;
use crate::game_logic_proxy::GameLogicProxy;
use crate::server_handler::ServerHandler;
#[tokio::main]
async fn main() {
    // Create a new connection to the server
    let server_connection = Arc::new(Mutex::new(ServerConnection::new("127.0.0.1:8080").expect("Failed to connect to server")));
    let server_handler = Arc::new(Mutex::new(ServerHandler::new(server_connection.clone())));
    let game_interface = GameInterface::new(server_handler.clone());
    let game_logic_proxy = Arc::new(Mutex::new(GameLogicProxy::new(server_handler, game_interface)));
    game_logic_proxy.start().await.unwrap();
}

