mod parsing;

#[cfg(test)]
mod test {
    use crate::{round::RoundTileClaimed, Board, DrawWall, Game, Hand, HandTile, Hands, Tile};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_draw_tile_from_wall_moves_tile() {
        let mut game = Game::new(None);
        game.table.draw_wall = DrawWall(vec![3, 4, 5]);
        Hands::default()
            .insert_ids("0", &[1])
            .insert_ids("1", &[2])
            .clone_into(&mut game.table.hands);

        let drawn_tile = game.draw_tile_from_wall();

        assert_eq!(game.table.draw_wall, DrawWall(vec![3, 4]));
        assert_eq!(drawn_tile, Some(5));
        assert_eq!(game.round.wall_tile_drawn, Some(5));
        assert_eq!(
            game.table.hands,
            Hands::default()
                .insert_ids("0", &[1, 5])
                .insert_ids("1", &[2])
                .to_owned()
        );
    }

    #[test]
    fn test_draw_tile_from_wall_returns_null() {
        let mut game = Game::new(None);
        let mut hands = Hands::default();
        hands.insert_ids("0", &[1]);
        hands.insert_ids("1", &[2]);
        let hands_clone = hands.clone();
        game.table.draw_wall = DrawWall::default();
        game.table.hands = hands;

        let drawn_tile = game.draw_tile_from_wall();

        assert_eq!(game.table.draw_wall, DrawWall::default());
        assert_eq!(drawn_tile, None);
        assert_eq!(game.round.wall_tile_drawn, None);
        assert_eq!(game.table.hands, hands_clone);
    }

    #[test]
    fn test_discard_tile_to_board() {
        let mut game = Game::new(None);
        game.table.board = Board(vec![16, 17, 18]);
        game.table.hands = {
            let mut hands = Hands::default();
            let mut player_a_tiles = vec![];
            for i in 1..15 {
                player_a_tiles.push(HandTile::from_id(i));
            }
            hands.insert("0", Hand(player_a_tiles));
            hands.insert("1", Hand(vec![HandTile::from_id(15)]));
            hands
        };

        let discarded_tile = game.discard_tile_to_board(&2);

        assert_eq!(game.table.board, Board(vec![16, 17, 18, 2]));
        assert_eq!(game.table.hands, {
            let mut hands = Hands::default();
            let mut player_a_tiles = vec![];
            for i in 1..15 {
                if i != 2 {
                    player_a_tiles.push(HandTile::from_id(i));
                }
            }
            hands.insert("0", Hand(player_a_tiles));
            hands.insert("1", Hand(vec![HandTile::from_id(15)]));
            hands
        });
        assert!(discarded_tile);
        assert_eq!(
            game.round.tile_claimed,
            Some(RoundTileClaimed {
                by: None,
                from: game.players.0[0].clone(),
                id: 2,
            })
        );
    }

    #[test]
    fn test_get_current_player() {
        let mut game = Game::new(None);
        game.round.player_index = 2;
        let player = game.get_current_player();
        assert_eq!(player, game.players.0[2]);
    }

    #[test]
    fn test_create_default_game() {
        let game = Game::new(None);

        assert_eq!(game.players.len(), 4);

        for player in game.players.iter() {
            let hand = game.table.hands.get(player);
            assert_eq!(hand.len(), 13);
            assert_eq!(game.score.get(player), Some(&0));
        }

        assert_eq!(game.table.draw_wall.len(), 144 - 4 * 13);
        assert_eq!(game.table.board.len(), 0);
    }

    #[test]
    fn test_game_print_summary() {
        let mut game = Game::new(None);

        game.table.hands.update_player_hand("0", "1B 2C,2C,2C");
        game.table.hands.update_player_hand("1", "2D");
        game.table.hands.update_player_hand("2", "3D");
        // Different order is then sorted
        game.table.hands.update_player_hand("3", "5D,4D");

        game.table.draw_wall.replace_tail_summary("5D,4D,3D,4D");
        game.table.board.push_by_summary("6D,4D,5D");

        game.start();

        assert_eq!(
            game.print_summary().trim(),
            r#"
- P1: 1B 2C,2C,2C
- P2: 2D
- P3: 3D
- P4: 4D,5D
Wall: 5D,4D,3D...
Board: 5D,4D...
Turn: P1, Dealer: P1, Round: 1, Wind: East, Phase: Playing
"#
            .trim()
        );
    }

    #[test]
    fn test_game_from_summary() {
        let game = Game::from_summary(
            r#"
- P1: 1B 2C,2C,2C
- P2: 2D
- P3: 3D
- P4: 4D,5D
Wall: 5D,4D,3D...
Board: 5D,4D...
Turn: P3, Dealer: P4, Round: 2, Wind: East, Phase: Playing
"#,
        );

        assert_eq!(
            game.table.draw_wall.0.last().unwrap().clone(),
            Tile::from_summary("5D").get_id()
        );

        assert_eq!(game.table.hands.get_player_hand_len("0"), 4);
        assert_eq!(game.table.hands.get_player_hand_len("1"), 1);
        assert_eq!(game.table.hands.get_player_hand_len("2"), 1);
        assert_eq!(game.table.hands.get_player_hand_len("3"), 2);
        assert_eq!(game.round.player_index, 2);
        assert_eq!(game.round.dealer_player_index, 3);
        assert_eq!(game.round.round_index, 1);
    }
}
