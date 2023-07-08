#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_validation() {
        let move = Move::new(Position::new(1, 1), Position::new(1, 2));
        assert!(MoveValidator::validate(&move));
    }
}
