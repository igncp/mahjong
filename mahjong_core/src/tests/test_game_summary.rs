#[cfg(test)]
mod test {
    use crate::{game_summary::GameSummary, hand::HandPossibleMeld, Game, Hand, Tile};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_possible_melds() {
        let mut game = Game::new(None);
        game.start_with_players();
        let first_player = game.players.first();
        let mut game_summary = GameSummary::from_game(&game, first_player).unwrap();

        game_summary.hand = Some(Hand::from_summary("一萬,一萬,一萬,三索,三索"));

        let discarded_tile = Tile::id_from_summary("三索");
        game_summary.board.0.push(discarded_tile);
        game_summary.round.discarded_tile = Some(discarded_tile);
        game_summary.round.player_index = 2;

        let possible_melds: Vec<String> = game_summary
            .get_possible_melds()
            .iter()
            .map(|meld| {
                let hand_possible_meld: HandPossibleMeld = meld.clone().into();

                hand_possible_meld.to_summary()
            })
            .collect();

        // One is own and one claimed
        assert_eq!(possible_melds, &["一萬,一萬,一萬 NO", "三索,三索,三索 NO"]);
    }
}
