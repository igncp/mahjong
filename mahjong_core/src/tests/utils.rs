use crate::{Game, Players};

impl Game {
    pub fn start_with_players(&mut self) {
        self.players = Players::default();
        self.players.push("0".to_string());
        self.players.push("1".to_string());
        self.players.push("2".to_string());
        self.players.push("3".to_string());
        self.start();
    }
}
