#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}
#[derive(Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Clone, Copy)]
pub struct Piece {
    _color: Color,
    _piece_type  : PieceType,
}
impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self { 
            _color: color, 
            _piece_type: piece_type,
        }
    }
    // other methods for piece mechanicss
}
