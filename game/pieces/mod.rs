mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
pub use bishop::Bishop;
pub use king::King;
pub use knight::Knight;
pub use pawn::Pawn;
pub use queen::Queen;
pub use rook::Rook;
use crate::game::Position;
use crate::game::Color;
pub trait Piece {
    fn valid_move(&self, dest: Position) -> bool;
    fn get_color(&self) -> Color;
    fn get_position(&self) -> Position;
    fn set_position(&mut self, position: Position);
}