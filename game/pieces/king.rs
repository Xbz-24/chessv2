use::crate::game::Position;
use::crate::game::Color;
use::crate::game::Piece;
pub struct King {
    pub position: Position,
    pub color: Color,
    pub has_moved: bool, // This is needed for castling logic
}
impl King {
    pub fn new (position: Position, color: Color) -> Self {
        Self { position, color, has_moved: false}
    }
}
impl Piece for King {
    fn valid_move(&self, dest: Position) -> bool {
        // A king can move one square in any direction.
        let dx=(self.position.x - dest.x).abs();
        let dy=(self.position.y - dest.y).abs();
        dx<=1&&dy<=1
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: Position) {
        self.position = position;
        self.has_moved = true;
    }
}