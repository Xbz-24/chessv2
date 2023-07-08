pub trait move_validator{
    fn validate(&self, game_move: Move, game_state: GameState)->bool;
}