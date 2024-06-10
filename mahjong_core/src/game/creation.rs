use super::{
    definition::{Game, GamePhase, GameStyle},
    Players,
};
use crate::{deck::DEFAULT_DECK, round::Round, Score};
use uuid::Uuid;

#[derive(Default, Clone)]
pub struct GameNewOpts {
    pub players: Option<Players>,
}

impl Game {
    pub fn new(opts: Option<GameNewOpts>) -> Self {
        let parsed_opts = opts.unwrap_or_default();
        let version = Uuid::new_v4().to_string();
        let mut players = parsed_opts.players.clone().unwrap_or_default();
        let game_style = GameStyle::HongKong;
        let players_num = Self::get_players_num(&game_style);

        if players.is_empty() {
            for player_id in 0..players_num {
                players.push(player_id.to_string());
            }
        }

        let table = DEFAULT_DECK.create_table(&players);
        let mut score = Score::default();

        for player_id in &players.0 {
            score.insert(player_id, 0);
        }

        Self {
            id: "game_id".to_string(),
            name: "game_name".to_string(),
            phase: GamePhase::Beginning,
            players,
            round: Round::new(&game_style),
            score,
            style: GameStyle::HongKong,
            table,
            version,
        }
    }
}
