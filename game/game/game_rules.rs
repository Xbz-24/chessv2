use crate::game::board::Board;
use crate::game::pieces::{Piece, Color, Position};

pub struct GameRules {
    pub board: Board,
}

impl GameRules {
    pub fn new() -> Self {
        let board = Board::new();
        Self { board }
    }

    // Verifies whether a move is valid or not for a given piece
    // If the move is not valid, return false
    pub fn validate_move<P: Piece>(&self, piece: &P, from: Position, dest: Position) -> bool {
        // Check if the destination is within the board bounds.
        if dest.x >= 8 || dest.y >= 8 {
            return false;
        }

        // Check if the destination is occupied by a piece of the same color
        if let Some(other) = self.board.get_piece(dest) {
            if other.get_color() == piece.get_color() {
                return false;
            }
        }

        // Check if the movement of the piece is valid.
        if !piece.valid_move(&self.board, dest) {
            return false;
        }

        true
    }
}
