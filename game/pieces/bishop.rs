use crate::game::Position;
use crate::game::Color;
use crate::pieces::Piece;

pub struct Bishop {
    pub position: Position,
    pub color: Color,
}

impl Bishop {
    pub fn new(position: Position, color: Color) -> Self {
        Self { position, color }
    }
}
impl Piece for Bishop {
    fn valid_move(&self, dest: Position) -> bool {
        // A bishop can move diagonally. This occurs when the change in the x and y psitions are equal either positive or negative
        let dx=(self.position.x - dest.x).abs();
        let dy=(self.position.y - dest.y).abs();
        dx==dy
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}
