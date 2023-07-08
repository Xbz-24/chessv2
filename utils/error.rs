#[derive(Debug)]
pub enum ChessError{
    InvalidMove,
    InvalidPosition,
    NoPieceAtPosition,
    WrongTurn,
    //Add more errors as needed
}
impl std::fmt::Display for ChessError{
    fn fmt(&self, f: &mut std::fmt::Formatter)->std::fmt::Result{
        match self{
            Self::InvalidMove=>write!(f, "Invalid move"),
            Self::InvalidPosition=>write!(f, "Invalid position"),
            Self::NoPieceAtPosition=>write!(f, "No piece at given position"),
            Self::WrongTurn=>write!(f, "Wrong turn"),
            // math other errors as needed
        }
    }
}
impl std::error::Error for ChessError{}