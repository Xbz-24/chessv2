use super::piece::Piece;

pub struct Game {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    // Other fields for game state, such as whose turn it is
}

impl Game {
    pub fn new() -> Self {
        // Initialize a game
        let board = [[None; 8]; 8];
        let turn = Color::White;
        Game { board, turn }
    }           
    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        // Check if the move is within the board boundaries
        if !self.is_valid_position(from_row, from_col) || !self.is_valid_position(to_row, to_col) {
            return Err(String::from("Invalid move: position out of bounds."));
        }
        // Retrieve the piece from the source position
        let piece = self.board[from_row][from_col].take();
        // Check if there is a piece at the source position
        if piece.color() != self.turn {
            return Err(InvalidMoveError::NotPlayerTurn));
        }
        // Check if the move is legal for the piece
        if !piece.unwrap().is_move_legal(from, to) {
            return Err(InvalidMoveError:IllegalMove));
        }
        // Move the piece to the destination position
        self.board[to_row][to_col] = piece;
        Ok(())
    }
    fn is_valid_position(&self, row: usize, col: usize) -> bool {
        row < 8 && col < 8
    }
    // other methods for game mechanics
}
