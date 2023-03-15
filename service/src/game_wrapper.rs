use mahjong_core::{Game, Player};
use uuid::Uuid;

pub fn create_game() -> Game {
    let mut players: Vec<Player> = vec![];
    for index in 0..4 {
        let player = Player {
            id: Uuid::new_v4().to_string(),
            name: format!("Custom Player {index}"),
        };

        players.push(player);
    }

    let mut game = Game {
        id: Uuid::new_v4().to_string(),
        name: "Custom Game".to_string(),
        ..Default::default()
    };

    game.set_players(&players);

    game
}
