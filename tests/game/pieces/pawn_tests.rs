#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_moves() {
        let piece = Pawn::new(Color::White);
        let position = Position::new(1, 1);
        let valid_moves = piece.valid_moves(&position);
        // test that the returned moves are valid for a pawn
    }
}
