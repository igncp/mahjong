#[cfg(test)]
mod test {
    use crate::deck::DEFAULT_DECK;

    #[test]
    fn test_deck_total_count() {
        let keys = DEFAULT_DECK.0.keys().len();
        assert_eq!(keys, 144);
    }

    #[test]
    fn test_create_table_counts() {
        let mut players = vec![];
        for num in 0..4 {
            let player = num.to_string();
            players.push(player);
        }
        let table = DEFAULT_DECK.create_table(&players);

        assert_eq!(table.board.len(), 0);
        assert_eq!(table.draw_wall.len(), 144 - 4 * 13);
        assert_eq!(table.hands.keys().len(), 4);
    }
}
