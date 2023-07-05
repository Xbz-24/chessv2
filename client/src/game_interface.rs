// File: game_interface.rs
use std::io::{self, Write};
use crate::connection::ServerConnection;
use crate::game_logic_proxy::GameLogicProxy;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct GameInterface {
    game_logic_proxy: Arc<Mutex<GameLogicProxy>>,
}
impl GameInterface {
    pub fn new(game_logic_proxy: Arc<Mutex<GameLogicProxy>>) -> Self {
        GameInterface {
            game_logic_proxy
        }
    }
    pub async fn start(&self) -> io::Result<()> {
        // Start receiving and printing server response
        let game_logic_proxy = self.game_logic_proxy.clone();
        tokio::spawn(async move {
            loop {
                if let Ok(response) = game_logic_proxy.lock().await.server_handler.lock().await.receive().await {
                    println!("Server response: {}", response);
                }
            }
        });

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            // send input to server
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() == 4 {
                let from = (parts[0].parse().unwrap(), parts[1].parse().unwrap());
                let to = (parts[2].parse().unwrap(), parts[3].parse().unwrap());
                self.game_logic_proxy.lock().await.make_move(from, to).await;
            }
        }
    }
    pub fn update_board(&self, _from: (usize, usize), _to: (usize, usize)) {
        
    }
}
