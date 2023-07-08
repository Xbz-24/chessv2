use crate::game::Position;
use crate::game::Color;
use crate::game::Board;
use crate::pieces::Piece;
pub struct Rook {
    pub position: Position.
    pub color: Color,
}
impl Rook {
    pub fn new(position: Position, color: Color) -> Self{
        Self {position, color}
    }
}
impl Piece for Rook{
    fn valid_move(&self, board: &Board, dest: Position) -> bool {
        let dx=(self.position.x - dest.x).abs() as isize;
        let dy=(self.position.y - dest.y).abs() as isize;
        // horizontal move
        if dx==0&&dy!=0{
            for 1 in 1..dy{
                let checking_position=Position{
                    x:self.position.x, 
                    y:(self.position.y as isize + i * direction.1) as usize
                };
                if board.is_occupied(checking_position){
                    return false;
                }
            }
            return true;
        }
        // vertical move
        if dx!=0&&dy==0{
            for 1 in 1..dx{
                let checking_position=Position{
                    x:(self.position.x as isize + i * direction.0) as usize, 
                    y:self.position.y
                };
                if board.is_occupied(checking_position){
                    return false;
                }
            }
            retrun true;
        }
        false
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_position(&self) -> Position {
        self.position
    }
    fn set_position(&mut self, position: Position) {
        self.position=position;
    }
}