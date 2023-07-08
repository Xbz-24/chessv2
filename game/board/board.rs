// src/game/board/board.rs

use crate::game::board::{Tile, Position, BOARD_SIZE};
use crate::game::pieces::{Piece, PieceType};
use std::option::Option;

pub struct Board {
    tiles: [[Tile; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        let tiles = [[Tile::new(); BOARD_SIZE]; BOARD_SIZE];
        Self { tiles }
    }

    pub fn init_board(&mut self) {
        // initialize the board with pieces
        // Use set_piece on the tiles where you want to put pieces
    }

    pub fn get_tile(&self, position: Position) -> Option<&Tile> {
        if self.valid_position(&position) {
            Some(&self.tiles[position.x][position.y])
        } else {
            None
        }
    }

    pub fn set_piece(&mut self, position: Position, piece: Option<Box<dyn Piece>>) {
        if self.valid_position(&position) {
            self.tiles[position.x][position.y].set_piece(piece);
        }
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), &'static str> {
        if self.valid_move(from, to) {
            let piece = self.tiles[from.x][from.y].get_piece().take();
            self.set_piece(to, piece);
            Ok(())
        } else {
            Err("Invalid move")
        }
    }

    fn valid_position(&self, position: &Position) -> bool {
        position.x >= 0 && position.x < BOARD_SIZE && position.y >= 0 && position.y < BOARD_SIZE
    }

    fn valid_move(&self, from: Position, to: Position) -> bool {
        if let Some(piece) = self.get_tile(from).and_then(|tile| tile.get_piece()) {
            if piece.can_move(from, to) {
                // For all pieces except for knights, check if there are any pieces in the way
                if piece.get_type() != PieceType::Knight {
                    if self.is_path_clear(from, to) {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }
        false
    }

    fn is_path_clear(&self, from: Position, to: Position) -> bool {
        // ... implement this function to check if the path is clear for a move ...
        // You will need to check all tiles between 'from' and 'to' to ensure that they do not contain pieces.
        // The specifics of this function will depend on your implementation of the game and pieces.
    }
}
