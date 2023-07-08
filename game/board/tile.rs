// src/game/board/tile.rs
use crate::game::pieces::Piece;
use std::option::Option;

pub struct Tile {
    piece: Option<Box<dyn Piece>>,
}

impl Tile {
    pub fn new() -> Self {
        Self { piece: None }
    }

    pub fn set_piece(&mut self, piece: Option<Box<dyn Piece>>) {
        self.piece = piece;
    }

    pub fn get_piece(&self) -> &Option<Box<dyn Piece>> {
        &self.piece
    }

    pub fn is_occupied(&self) -> bool {
        self.piece.is_some()
    }
}
