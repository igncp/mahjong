#[cfg(test)]
mod test {
    use crate::{game_summary::GameSummary, Game, Hand, Tile};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_possible_melds() {
        let game = Game::new(None);
        let first_player = game.players.first();
        let mut game_summary = GameSummary::from_game(&game, first_player).unwrap();

        game_summary.hand = Hand::from_summary("1C,1C,1C,3B,3B");

        let discarded_tile = Tile::id_from_summary("3B");
        game_summary.board.0.push(discarded_tile);
        game_summary.round.discarded_tile = Some(discarded_tile);
        game_summary.round.player_index = 2;

        let possible_melds = game_summary.get_possible_melds();

        // One is own and one claimed
        assert_eq!(possible_melds.len(), 2);
    }
}
