mod game_state;
mod game_rules;
mod history;

pub use self::{
    game_state::GameState,
    game_rules::GameRules,
    history::History,
};

use crate::game::board::Board;
use crate::game::moves::Move;
use crate::player::Player;
use std::sync::{Arc, Mutex};

pub struct Game {
    state: Arc<Mutex<GameState>>,
    rules: GameRules,
    history: History,
    players: Vec<Player>,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        // implement initialization of game components
    }

    pub fn start(&self) {
        // implement game start logic
    }

    pub fn update_state(&mut self, mv: Move) {
        // implement logic to update state
    }

    pub fn end_game(&mut self) {
        // implement game end logic
    }
}
