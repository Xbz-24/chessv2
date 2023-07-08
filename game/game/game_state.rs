// use crate::player::Player;
// use crate::game::Color;
// use crate::game::Board;
// pub struct GameState {
//     pub current_player: Color,
//     pub board: Board,
//     pub players: [Player; 2],
// }
// impl GameState {
//     pub fn new(player1: Player, player2: Player) -> Self {
//         let board = Board::new();
//         Self{
//             current_player: Color::White,
//             board,
//             players: [player1, player2],
//         }
//     }
//     // switches the current player
//     pub fn switch_player(&mut self){
//         self.current_player = match self.current_player {
//             Color::White => Color::Black,
//             Color::Black => Color::White,
//         };
//     }
//     // get the current player
//     pub fn get_current_player(&self)->&Player{
//         match self.current_player {
//             Color::White => &self.players[0],
//             Color::Black => &self.players[1],
//         }
//     }
//     // get mutable reference to the current player
//     pub fn get_current_player_mut(&mut self)->&mut Player{
//         match self.current_player{
//             Color::White => &mut self.players[0],
//             Color::Black => &mut self.players[1],
//         }
//     }
// }
// pub enum GameState{
//     NotStarted,
//     OnGoing,
//     Checkmate(color),
//     Stalemate,
// }
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
