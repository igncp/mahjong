#[cfg(test)]
mod test {
    use rustc_hash::FxHashMap;

    use crate::{round::RoundTileClaimed, Game, Hand, HandTile};

    #[test]
    fn test_draw_tile_from_wall_moves_tile() {
        let draw_wall = vec![3, 4, 5];
        let mut hands = FxHashMap::default();
        let mut game = Game::default();
        game.table.draw_wall = draw_wall;
        hands.insert(game.players[0].clone(), Hand(vec![HandTile::from_id(1)]));
        hands.insert(game.players[1].clone(), Hand(vec![HandTile::from_id(2)]));
        game.table.hands = hands;

        let drawn_tile = game.draw_tile_from_wall();

        assert_eq!(game.table.draw_wall, vec![3, 4]);
        assert_eq!(drawn_tile, Some(5));
        assert_eq!(game.round.wall_tile_drawn, Some(5));
        assert_eq!(game.table.hands, {
            let mut expected_hands = FxHashMap::default();
            expected_hands.insert(
                game.players[0].clone(),
                Hand(vec![HandTile::from_id(1), HandTile::from_id(5)]),
            );
            expected_hands.insert(game.players[1].clone(), Hand(vec![HandTile::from_id(2)]));
            expected_hands
        });
    }

    #[test]
    fn test_draw_tile_from_wall_returns_null() {
        let draw_wall = vec![];
        let mut game = Game::default();
        let mut hands = FxHashMap::default();
        hands.insert(game.players[0].clone(), Hand(vec![HandTile::from_id(1)]));
        hands.insert(game.players[1].clone(), Hand(vec![HandTile::from_id(2)]));
        game.table.draw_wall = draw_wall;
        game.table.hands = hands;
        let drawn_tile = game.draw_tile_from_wall();

        assert_eq!(game.table.draw_wall, vec![]);
        assert_eq!(drawn_tile, None);
        assert_eq!(game.round.wall_tile_drawn, None);
        assert_eq!(game.table.hands, {
            let mut expected_hands = FxHashMap::default();
            expected_hands.insert(game.players[0].clone(), Hand(vec![HandTile::from_id(1)]));
            expected_hands.insert(game.players[1].clone(), Hand(vec![HandTile::from_id(2)]));
            expected_hands
        });
    }

    #[test]
    fn test_discard_tile_to_board() {
        let mut game = Game::default();
        game.table.board = vec![16, 17, 18];
        game.table.hands = {
            let mut hands = FxHashMap::default();
            let mut player_a_tiles = vec![];
            for i in 1..15 {
                player_a_tiles.push(HandTile::from_id(i));
            }
            hands.insert(game.players[0].clone(), Hand(player_a_tiles));
            hands.insert(game.players[1].clone(), Hand(vec![HandTile::from_id(15)]));
            hands
        };

        let discarded_tile = game.discard_tile_to_board(&2);

        assert_eq!(game.table.board, vec![16, 17, 18, 2]);
        assert_eq!(game.table.hands, {
            let mut hands = FxHashMap::default();
            let mut player_a_tiles = vec![];
            for i in 1..15 {
                if i != 2 {
                    player_a_tiles.push(HandTile::from_id(i));
                }
            }
            hands.insert(game.players[0].clone(), Hand(player_a_tiles));
            hands.insert(game.players[1].clone(), Hand(vec![HandTile::from_id(15)]));
            hands
        });
        assert!(discarded_tile);
        assert_eq!(
            game.round.tile_claimed,
            Some(RoundTileClaimed {
                by: None,
                from: game.players[0].clone(),
                id: 2,
            })
        );
    }

    #[test]
    fn test_get_current_player() {
        let mut game = Game::default();
        game.round.player_index = 2;
        let player = game.get_current_player();
        assert_eq!(player, game.players[2]);
    }

    #[test]
    fn test_create_default_game() {
        let game = Game::default();

        assert_eq!(game.players.len(), 4);

        for player in game.players.iter() {
            let hand = game.table.hands.get(player).unwrap();
            assert_eq!(hand.0.len(), 13);
            assert_eq!(game.score.get(player), Some(&0));
        }

        assert_eq!(game.table.draw_wall.len(), 144 - 4 * 13);
        assert_eq!(game.table.board.len(), 0);
    }
}
