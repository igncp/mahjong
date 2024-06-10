#[cfg(test)]
mod test {
    use crate::{
        deck::DEFAULT_DECK,
        game::{GameStyle, Players},
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
        let table = DEFAULT_DECK.create_table(&players);

        assert_eq!(table.board.0.len(), 0);
        assert_eq!(table.draw_wall.0.len(), 144 - 4 * 13);
        assert_eq!(table.hands.0.keys().len(), 4);
    }
}
