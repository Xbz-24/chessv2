use super::super::super::game::Game;
use super::super::super::player::Player;
use super::super::super::moves::Move;
use super::super::super::board::Position;
use super::super::super::pieces::{Piece, Pawn, Rook, Knight, Bishop, Queen, King};

#[test]
fn test_game_initialization() {
    let game = Game::new();
    assert_eq!(game.players.len(), 2);
    assert_eq!(game.state.status, GameStatus::NotStarted);
    assert_eq!(game.board.tiles.len(), 8);
}

#[test]
fn test_valid_move() {
    let mut game = Game::new();
    let start = Position::new(1, 2); // assuming this is a valid start position
    let end = Position::new(1, 3); // assuming this is a valid end position
    let player = game.players[0].clone();

    let game_move = Move::new(start, end, player);
    assert!(game.validate_move(&game_move));
}

#[test]
fn test_invalid_move() {
    let mut game = Game::new();
    let start = Position::new(1, 2); // assuming this is a valid start position
    let end = Position::new(8, 8); // assuming this is an invalid end position for the piece at (1,2)
    let player = game.players[0].clone();

    let game_move = Move::new(start, end, player);
    assert!(!game.validate_move(&game_move));
}

#[test]
fn test_move_changes_game_state() {
    let mut game = Game::new();
    let start = Position::new(1, 2); // assuming this is a valid start position
    let end = Position::new(1, 3); // assuming this is a valid end position
    let player = game.players[0].clone();

    let game_move = Move::new(start, end, player);
    game.make_move(game_move);

    let moved_piece = game.board.get_tile(end).unwrap().piece;
    assert_eq!(moved_piece.unwrap().position, end);
}

#[test]
fn test_game_over() {
    let mut game = Game::new();
    // Simulate a situation where one player has lost (i.e., their king has been captured)
    game.state.status = GameStatus::Over;
    assert!(game.is_over());
}
