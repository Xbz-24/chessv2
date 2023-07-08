use crate::game::Position;
use crate::game::Color;
use crate::game::Piece;

pub struct Pawn {
    pub position: Position,
    pub color: Color,
    pub has_moved: bool, // This is needed for en passant logic
}
impl Pawn {
    pub fn new(position: Position, color: Color) -> Self {
        Self { position, color, has_moved: false }
    }
}
impl Piece for Pawn {
    fn valid_move(&self, dest: Position) -> bool {
        let dx=(self.position.x - dest.x).abs();
        let dy=(self.position.y - dest.y).abs();
        // Handle move direction based on color
        let direction=match self.color {
            Color:white => -1,
            Color:black => 1,
        };
        // Regular move (one square forward)
        if dx==0 && dy==direction {
            return !board.is_occupied(dest);
        }
        // First move (2 forward)
        if !self.has_moved&&dx==0&&dy==2*direction {
            // Move is invalid if there's a piece in the destination or the square in between
            let between = Position {x: self.position.x, y: self.position.y+direction};
            return !board.is_occupied(dest)&&!board.is_occupied(between);
        }
        // Capture move (diagonal forward)
        if dx==1&&dy==direction{
            // move is invalid fi there's no piece to capture
            return board_is_occupied(dest) && board.get_piece(dest).unwrap().get_color() != self.color;
        }
        false
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&mut self, new_position: Position) {
        self.position = new_position;
        self.has_moved = true;
    }
}
