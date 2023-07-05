#[derive(Debug, Clone, Copy)]
pub struct Bishop {
    color: Color,
}
impl Bishop {
    pub fn new(color: Color) -> Self {
        Bishop { color }
    }

    pub fn color(&self) -> Color {
        self.color
    }
    pub fn valid_moves(&self, position: Square, board: &Board) -> Vec<Square> {
        let mut moves = Vec::new();
        let directions = [(1,1), (1,-1), (-1,1), (-1,-1)]
        for &(dx, dy) in directions.iter() {
            let mut x = position.file() as i32 + dx
        }
    }
}