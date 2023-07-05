pub struct Player {
    pub name: String,
    pub score: i32,
    pub health: i32,
    pub level: i32,
}

impl Player {
    // Create a new Player
    pub fn new(name: String) -> Self {
        Player {
            name,
            score: 0,
            health: 100,
            level: 1,
        }
    }

    // A method for the player to gain points
    pub fn gain_score(&mut self, points: i32) {
        self.score += points;
    }

    // A method for the player to take damage
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health < 0 {
            self.health = 0;
        }
    }

    // A method for the player to level up
    pub fn level_up(&mut self) {
        self.level += 1;
        self.health = self.level * 100;
    }
}

