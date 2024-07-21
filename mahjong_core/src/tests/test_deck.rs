#[cfg(test)]
mod test {
    use crate::{
        deck::DEFAULT_DECK,
        game::{GameStyle, Players},
        table::PositionTilesOpts,
        Game,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deck_total_count() {
        let keys = DEFAULT_DECK.0.keys().len();
        assert_eq!(keys, 144);
    }

    #[test]
    fn test_create_table_counts() {
        let mut players = Players::default();
        for num in 0..Game::get_players_num(&GameStyle::HongKong) {
            let player = num.to_string();
            players.push(player);
        }
        let mut table = DEFAULT_DECK.create_table(&players);
        table.draw_wall.position_tiles(None);

        assert_eq!(table.board.0.len(), 0);
        assert_eq!(table.draw_wall.len(), 144);
        assert_eq!(table.hands.0.keys().len(), 4);

        let mut table_dead_wall = DEFAULT_DECK.create_table(&players);
        table_dead_wall
            .draw_wall
            .position_tiles(Some(PositionTilesOpts {
                shuffle: None,
                dead_wall: Some(true),
            }));
        assert_eq!(table_dead_wall.board.0.len(), 0);
        assert_eq!(table_dead_wall.draw_wall.len(), 144 - 14);
        assert_eq!(table_dead_wall.hands.0.keys().len(), 4);
    }
}
