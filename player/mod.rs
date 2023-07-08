// src/player/mod.rs

pub mod human;
pub mod ai;

use crate::game::moves::Move;
use crate::game::pieces::PieceColor;

pub trait Player {
    fn new(color: PieceColor) -> Self;
    fn make_move(&self) -> Move;
    fn get_color(&self) -> &PieceColor;
}

