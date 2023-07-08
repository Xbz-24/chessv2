use super::{Board, Tile};

#[test]
fn test_board_creation() {
    let board = Board::new();
    assert_eq!(board.tiles.len(), 8);
    for row in board.tiles {
        assert_eq!(row.len(), 8);
    }
}
