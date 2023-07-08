use crate::game::game::History;
use crate::game::moves::Move;
use crate::game::pieces::{Piece, PieceColor, PieceType};
use crate::game::board::Position;

// Test initialization of History
#[test]
fn test_history_init() {
    let history = History::new();
    assert!(history.moves.is_empty());
}

// Test adding a move to the history
#[test]
fn test_add_move() {
    let mut history = History::new();
    let piece = Piece::new(PieceColor::White, PieceType::Pawn);
    let move_to_add = Move::new(piece, Position::new(1, 2), Position::new(1, 3));

    history.add_move(move_to_add.clone());
    assert_eq!(history.moves.len(), 1);
    assert_eq!(history.moves[0], move_to_add);
}

// Test retrieving the last move
#[test]
fn test_get_last_move() {
    let mut history = History::new();
    let piece1 = Piece::new(PieceColor::White, PieceType::Pawn);
    let move1 = Move::new(piece1, Position::new(1, 2), Position::new(1, 3));
    history.add_move(move1.clone());

    let piece2 = Piece::new(PieceColor::Black, PieceType::Pawn);
    let move2 = Move::new(piece2, Position::new(6, 7), Position::new(6, 6));
    history.add_move(move2.clone());

    assert_eq!(history.get_last_move(), Some(move2));
}

// Test retrieving a move at a certain index
#[test]
fn test_get_move_at_index() {
    let mut history = History::new();
    let piece1 = Piece::new(PieceColor::White, PieceType::Pawn);
    let move1 = Move::new(piece1, Position::new(1, 2), Position::new(1, 3));
    history.add_move(move1.clone());

    let piece2 = Piece::new(PieceColor::Black, PieceType::Pawn);
    let move2 = Move::new(piece2, Position::new(6, 7), Position::new(6, 6));
    history.add_move(move2.clone());

    assert_eq!(history.get_move_at_index(0), Some(move1));
    assert_eq!(history.get_move_at_index(1), Some(move2));
    assert_eq!(history.get_move_at_index(2), None);
}
