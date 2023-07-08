pub fn convert_chess_coords_to_row_col(chess_coords: &str) -> Result<(usize, usize), GameError> {
    if chess_coords.len() != 2 {
        return Err(GameError::InvalidMove);
    }

    let file = chess_coords.chars().nth(0).unwrap();
    let rank = chess_coords.chars().nth(1).unwrap();

    let row = 8 - rank.to_digit(10).ok_or(GameError::InvalidMove)? as usize;
    let col = file as usize - 'a' as usize;

    Ok((row, col))
}
