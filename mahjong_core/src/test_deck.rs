#[cfg(test)]
mod test {
    use crate::deck::{create_table, get_default_deck};
    use crate::Player;

    #[test]
    fn test_deck_total_count() {
        let deck = get_default_deck();
        let keys = deck.keys().len();
        assert_eq!(keys, 144);
    }

    #[test]
    fn test_create_table_counts() {
        let deck = get_default_deck();
        let mut players = vec![];
        for num in 0..4 {
            let player = Player {
                id: num.to_string(),
                name: format!("Player {num}"),
            };
            players.push(player);
        }
        let table = create_table(&deck, &players);

        assert_eq!(table.board.len(), 0);
        assert_eq!(table.draw_wall.len(), 144 - 4 * 13);
        assert_eq!(table.hands.keys().len(), 4);
    }
}
