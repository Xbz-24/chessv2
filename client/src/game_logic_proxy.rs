use std::sync::Arc;
use tokio::sync::Mutex;

use crate::server_handler::ServerHandler;
use crate::game_interface::GameInterface;

pub struct GameLogicProxy {
    server_handler: Arc<Mutex<ServerHandler>>,
    game_interface: GameInterface,
}

impl GameLogicProxy {
    pub fn new(server_handler: Arc<Mutex<ServerHandler>>, game_interface: GameInterface) -> Self {
        GameLogicProxy {
            server_handler,
            game_interface,
        }
    }

    // Define methods to interact with the game logic via the server handler and game interface
    // For example:
    pub async fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        // Send the move request to the server handler
        self.server_handler.lock().await.send_move_request(from, to).await.unwrap();
        // Update the game interface or perform other necessary actions
        self.game_interface.update_board(from, to);
    }
}
