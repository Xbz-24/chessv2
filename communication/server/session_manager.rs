use std::collections::HashMap;
use crate::game::game::Game;

pub struct SessionManager {
    sessions: HashMap<u32, Game>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    // More methods here...
}
