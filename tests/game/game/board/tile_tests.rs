#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_creation() {
        let tile = Tile::new();
        assert!(tile.piece.is_none());
    }
}
