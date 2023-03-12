use std::collections::HashMap;

use mahjong_core::{
    deck::{create_table, get_default_deck},
    Game, GamePhase, Player, Score,
};
use uuid::Uuid;

pub fn create_game() -> Game {
    let id = Uuid::new_v4();
    let mut players: Vec<Player> = vec![];
    let mut score: Score = HashMap::new();
    for index in 0..4 {
        let player = Player {
            id: Uuid::new_v4().to_string(),
            name: format!("Player {index}"),
        };

        players.push(player);
    }

    for player in players.iter() {
        score.insert(player.id.clone(), 0);
    }

    let deck = get_default_deck();
    let table = create_table(&deck, &players);

    Game {
        deck,
        id: id.to_string(),
        name: "Service Game".to_string(),
        phase: GamePhase::Beginning,
        players,
        score,
        table,
    }
}
