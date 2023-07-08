pub trait MoveGenerator{
    fn generate_moves(&self, game_state: GameState, player: Player)->Vec<Move>;
}