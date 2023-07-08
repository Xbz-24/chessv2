// src/player/ai/ai_player.rs

use crate::player::Player;
use crate::game::moves::Move;
use crate::game::pieces::PieceColor;
use super::move_picker::MovePicker;

pub struct AIPlayer {
    color: PieceColor,
    move_picker: MovePicker,
}

impl Player for AIPlayer {
    fn new(color: PieceColor) -> AIPlayer {
        AIPlayer {
            color,
            move_picker: MovePicker::new(),
        }
    }

    fn make_move(&self) -> Move {
        self.move_picker.pick_move()
    }

    fn get_color(&self) -> &PieceColor {
        &self.color
    }
}

