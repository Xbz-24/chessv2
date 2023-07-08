// src/player/human/human_player.rs

use crate::player::Player;
use crate::game::moves::Move;
use crate::game::pieces::PieceColor;
use super::move_maker::MoveMaker;

pub struct HumanPlayer {
    color: PieceColor,
    move_maker: MoveMaker,
}

impl Player for HumanPlayer {
    fn new(color: PieceColor) -> HumanPlayer {
        HumanPlayer {
            color,
            move_maker: MoveMaker::new(),
        }
    }

    fn make_move(&self) -> Move {
        self.move_maker.make_move()
    }

    fn get_color(&self) -> &PieceColor {
        &self.color
    }
}

