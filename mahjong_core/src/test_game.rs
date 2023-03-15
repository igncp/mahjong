#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        game::{
            discard_tile_to_board, draw_tile_from_wall, DiscardTileToBoardOpts,
            DrawTileFromWallOpts,
        },
        Game, HandTile, Round, RoundTileClaimed,
    };

    #[test]
    fn test_draw_tile_from_wall_moves_tile() {
        let draw_wall = vec![3, 4, 5];
        let mut hands = HashMap::new();
        hands.insert("playerA".to_string(), vec![HandTile::from_id(1)]);
        hands.insert("playerB".to_string(), vec![HandTile::from_id(2)]);
        let round = Round::default();
        let mut opts = DrawTileFromWallOpts {
            draw_wall,
            hands,
            round,
            player_id: "playerA".to_string(),
        };
        let drawn_tile = draw_tile_from_wall(&mut opts);

        assert_eq!(opts.draw_wall, vec![3, 4]);
        assert_eq!(drawn_tile, Some(5));
        assert_eq!(opts.round.wall_tile_drawn, Some(5));
        assert_eq!(opts.hands, {
            let mut hands = HashMap::new();
            hands.insert(
                "playerA".to_string(),
                vec![HandTile::from_id(1), HandTile::from_id(5)],
            );
            hands.insert("playerB".to_string(), vec![HandTile::from_id(2)]);
            hands
        });
    }

    #[test]
    fn test_draw_tile_from_wall_returns_null() {
        let draw_wall = vec![];
        let mut hands = HashMap::new();
        hands.insert("playerA".to_string(), vec![HandTile::from_id(1)]);
        hands.insert("playerB".to_string(), vec![HandTile::from_id(2)]);
        let round = Round::default();
        let mut opts = DrawTileFromWallOpts {
            draw_wall,
            hands,
            round,
            player_id: "playerA".to_string(),
        };
        let drawn_tile = draw_tile_from_wall(&mut opts);

        assert_eq!(opts.draw_wall, vec![]);
        assert_eq!(drawn_tile, None);
        assert_eq!(opts.round.wall_tile_drawn, None);
        assert_eq!(opts.hands, {
            let mut hands = HashMap::new();
            hands.insert("playerA".to_string(), vec![HandTile::from_id(1)]);
            hands.insert("playerB".to_string(), vec![HandTile::from_id(2)]);
            hands
        });
    }

    #[test]
    fn test_discard_tile_to_board() {
        let board = vec![16, 17, 18];
        let round = Round::default();
        let hands = {
            let mut hands = HashMap::new();
            let mut player_a_tiles = vec![];
            for i in 1..15 {
                player_a_tiles.push(HandTile::from_id(i));
            }
            hands.insert("playerA".to_string(), player_a_tiles);
            hands.insert("playerB".to_string(), vec![HandTile::from_id(15)]);
            hands
        };

        let mut opts = DiscardTileToBoardOpts {
            board,
            hands,
            player_id: "playerA".to_string(),
            round,
            tile_id: 2,
        };
        let discarded_tile = discard_tile_to_board(&mut opts);

        assert_eq!(opts.board, vec![16, 17, 18, 2]);
        assert_eq!(opts.hands, {
            let mut hands = HashMap::new();
            let mut player_a_tiles = vec![];
            for i in 1..15 {
                if i != 2 {
                    player_a_tiles.push(HandTile::from_id(i));
                }
            }
            hands.insert("playerA".to_string(), player_a_tiles);
            hands.insert("playerB".to_string(), vec![HandTile::from_id(15)]);
            hands
        });
        assert_eq!(discarded_tile, Some(2));
        assert_eq!(
            opts.round.tile_claimed,
            Some(RoundTileClaimed {
                by: None,
                from: "playerA".to_string(),
                id: 2,
            })
        );
    }

    #[test]
    fn test_get_current_player() {
        let mut game = Game::default();
        game.round.player_index = 2;
        let player = game.get_current_player();
        assert_eq!(player.name, "Player 2");
    }

    #[test]
    fn test_create_default_game() {
        let game = Game::default();

        assert_eq!(game.players.len(), 4);
        assert_eq!(game.deck.0.keys().len(), 144);

        for player in game.players.iter() {
            let hand = game.table.hands.get(&player.id).unwrap();
            assert_eq!(hand.len(), 13);
            assert_eq!(game.score.get(&player.id), Some(&0));
        }

        assert_eq!(game.table.draw_wall.len(), 144 - 4 * 13);
        assert_eq!(game.table.board.len(), 0);
    }
}
