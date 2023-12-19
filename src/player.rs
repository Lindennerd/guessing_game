use std::cmp::Ordering;

pub struct Player {
    name: String,
    record: u32,
    games_played: u32,
    score: u32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            record: 0,
            games_played: 0,
            score: 0,
        }
    }

    pub fn add_new_game(&mut self) {
        self.games_played += 1;
    }

    pub fn add_new_win(&mut self, guesses: u32) {
        match guesses.cmp(&self.record) {
            Ordering::Less => {
                self.record = guesses;
            }
            _ => {}
        };
    }

    pub fn add_new_score(&mut self, guesses: u32) {
        self.score += guesses * 100;
    }
}
