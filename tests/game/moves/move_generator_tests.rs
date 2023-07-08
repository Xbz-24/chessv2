#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_generation() {
        let position = Position::new(1, 1);
        let moves = MoveGenerator::generate(&position);
        // test that the returned moves are valid for the given position
    }
}
