use crate::player::Player;
use crate::pieces::Piece;
use crate::Color;

pub struct GameState {
    pub board: [[Option<Piece>; 8]; 8],
    pub players: [Player; 2],
    pub turn: Color,
}

impl GameState {
        pub fn new(player1: Player, player2: Player) -> Self {
            let board = [[None; 8]; 8];
            let players = [player1, player2];
            let turn = Color::White;
            GameState { board, players, turn}
        }
        pub fn switch_turn(&mut self) {
            self.turn = match self.turn {
                Color::White => Color::Black,
                Color::Black => Color::White,
            }
        }
}
