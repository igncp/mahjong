#[cfg(test)]
mod test {
    use crate::{game_summary::GameSummary, Game, HandTile};

    #[test]
    fn test_get_possible_melds() {
        let game = Game::default();
        let first_player = game.players.first().unwrap();
        let mut game_summary = GameSummary::from_game(&game, first_player).unwrap();

        game_summary.hand.0 = vec![76, 44, 10, 45, 82, 84, 64, 65, 134, 51, 119, 59, 37]
            .into_iter()
            .map(|id| HandTile {
                id,
                concealed: true,
                set_id: None,
            })
            .collect();
        game_summary.board = vec![112];
        game_summary.round.discarded_tile = Some(112);
        game_summary.round.player_index = 2;

        let possible_melds = game_summary.get_possible_melds();

        // One is own and one claimed
        assert_eq!(possible_melds.len(), 2);
    }
}
