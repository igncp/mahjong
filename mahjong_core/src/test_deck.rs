#[cfg(test)]
mod test {
    use crate::{Deck, Player};

    #[test]
    fn test_deck_total_count() {
        let deck = Deck::default();
        let keys = deck.0.keys().len();
        assert_eq!(keys, 144);
    }

    #[test]
    fn test_create_table_counts() {
        let deck = Deck::default();
        let mut players = vec![];
        for num in 0..4 {
            let player = Player {
                id: num.to_string(),
                name: format!("Player {num}"),
            };
            players.push(player);
        }
        let table = deck.create_table(&players);

        assert_eq!(table.board.len(), 0);
        assert_eq!(table.draw_wall.len(), 144 - 4 * 13);
        assert_eq!(table.hands.keys().len(), 4);
    }
}
