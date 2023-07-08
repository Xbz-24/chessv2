use crate::game::Position;
use crate::game::Color;
use crate::pieces::Piece;
pub struct Knight {
    pub position: Position,
    pub color: Color,
}
impl Knight {
    pub fn new(position: Position, color: Color) -> Self {
        Self { position, color }
    }
}
impl Piece for Knight {
    fn valid_move(&self, dest: Position) -> bool {
        // A knight can move two squares in one direction and then one square in a perpendicular direction. This is a total of three squares moved.
        let dx=(self.position.x - dest.x).abs();
        let dy=(self.position.y - dest.y).abs();
        (dx==2&&dy==1)||(dx==1&&dy==2)
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