use::game_logic::player::Player;    
use game_logic::game::Game;

pub struct GameSession<Player> {
    // game state fields
    game: Game,
    players: Vec<Player>,
    current_turn: usize,
}
impl <Player> GameSession <Player> {
    pub fn new(game: Game, players: Vec<Player>) -> Self {
        GameSession {
            game,
            players,
            current_turn: 0,
        }
    }
    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        // Perform move logic here
        let piece = self.game.board[from.0][from.1];
        self.game.board[to.0][to.1] = piece;
        self.game.board[from.0][from.1] = None;
    }

    // Add other game handling methods as needed
    // For example:
    pub fn get_current_turn_player(&self) -> Option<&Player> {
        self.players.get(self.current_turn)
    }
}
