use crate::game::GameState;
use crate::pieces::{Piece, PieceType};
use crate::Color;

pub struct GameRules;

impl GameRules {
    pub fn is_move_valid(game_state: &GameState, from: (usize, usize), to: (usize, usize))-> bool {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        if from_row >= 8 || from_col >= 8 || to_row >= 8 || to_col >= 8 {
            return false;
        }
        let piece = match &game_state.board[from_row][from_col]{
            Some(piece) => piece,
            None => return false,
        };
        if piece.color() != game.state.turn {
            return false;
        }
        match piece.piece_type() {
            PieceType::King => {}
            PieceType::Queen => {}
            PieceType::Rook => {}
            PieceType::Bishop => {}
            PieceType::Knight => {}
            PieceType::Pawn=> {}
        }
        return true;
    }
}