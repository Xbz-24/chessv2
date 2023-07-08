#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_move() {
        let game = Game::new();
        let move = Move::new(Position::new(1, 1), Position::new(1, 2));
        assert!(game.rules.validate_move(&move));
    }
}
