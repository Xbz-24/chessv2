// tests/game/game/game_state_tests.rs

use crate::game::game_state::{GameState, GameStatus};

#[test]
fn test_game_state_creation() {
    let game_state = GameState::new();

    assert_eq!(game_state.game_status, GameStatus::NotStarted);
    assert_eq!(game_state.board.len(), 8);
    assert_eq!(game_state.board[0].len(), 8);
}

#[test]
fn test_game_start() {
    let mut game_state = GameState::new();
    game_state.start_game();

    assert_eq!(game_state.game_status, GameStatus::Ongoing);
    // Assert that your board is initialized properly
}

#[test]
fn test_game_finish() {
    let mut game_state = GameState::new();
    game_state.start_game();
    game_state.finish_game();

    assert_eq!(game_state.game_status, GameStatus::Finished);
    // Assert any other changes that should occur when the game is finished
}
