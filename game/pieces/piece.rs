use crate::game::Position;
use crate::game::Color;
use crate::game::Board;

pub trait Piece {
    fn valid_move(&self, board: &Board, dest: Position) -> bool;
    fn get_color(&self) -> Color;
    fn get_position(&self) -> Position;
    fn set_position(&mut self, new_position: Position);
}