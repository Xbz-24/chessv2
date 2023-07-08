#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_creation() {
        let move = Move::new(Position::new(1, 1), Position::new(1, 2));
        assert_eq!(move.start, Position::new(1, 1));
        assert_eq!(move.end, Position::new(1, 2));
    }
}
