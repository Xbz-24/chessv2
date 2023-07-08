#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_transition() {
        let mut game = Game::new();
        assert_eq!(game.state, GameState::NOT_STARTED);
        game.start();
        assert_eq!(game.state, GameState::ONGOING);
        game.end();
        assert_eq!(game.state, GameState::FINISHED);
    }
}
