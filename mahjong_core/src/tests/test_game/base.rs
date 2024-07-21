#[cfg(test)]
mod test {
    use crate::{
        deck::DEFAULT_DECK, game::DrawTileResult, round::RoundTileClaimed, Board, DrawWall, Game,
        Hands, Tile,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_draw_tile_from_wall_moves_tile() {
        let mut game = Game::new(None);
        game.table.draw_wall.position_tiles(None);
        let player_wind = game.get_player_wind();
        game.table
            .draw_wall
            .replace_tail_summary(&player_wind, "五筒");
        game.table
            .hands
            .update_players_hands(&["一筒", "二筒", "", ""]);

        let drawn_tile = game.draw_tile_from_wall();
        let expected_drawn_tile = Tile::from_summary("五筒").get_id();

        assert_eq!(drawn_tile, DrawTileResult::Normal(expected_drawn_tile));
        assert_eq!(game.round.wall_tile_drawn, Some(expected_drawn_tile));
        assert_eq!(
            game.table.hands,
            Hands::default()
                .update_players_hands(&["一筒,五筒", "二筒", "", ""])
                .to_owned()
        );
    }

    #[test]
    fn test_draw_tile_from_wall_returns_null() {
        let mut game = Game::new(None);
        game.table
            .hands
            .update_players_hands(&["一筒", "二筒", "", ""]);
        let hands_clone = game.table.hands.clone();
        game.table.draw_wall = DrawWall::default();

        let drawn_tile = game.draw_tile_from_wall();

        assert_eq!(game.table.draw_wall, DrawWall::default());
        assert_eq!(drawn_tile, DrawTileResult::WallExhausted);
        assert_eq!(game.round.wall_tile_drawn, None);
        assert_eq!(game.table.hands, hands_clone);
    }

    #[test]
    fn test_discard_tile_to_board() {
        let mut game = Game::new(None);
        game.table.board = Board::from_summary("一筒,二筒,三筒");
        game.table.hands.update_players_hands(&[
            "一萬,二萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一索,二索,三索,四索,五索",
            "八筒",
            "九筒",
            "",
        ]);

        let discarded_tile = game.discard_tile_to_board(&Tile::id_from_summary("二萬"));

        assert_eq!(game.table.board.to_summary(), "一筒,二筒,三筒,二萬");
        assert_eq!(
            game.table.hands,
            Hands::default()
                .update_players_hands(&[
                    "一萬,三萬,四萬,五萬,六萬,七萬,八萬,九萬,一索,二索,三索,四索,五索",
                    "八筒",
                    "九筒",
                    "",
                ])
                .to_owned()
        );
        assert!(discarded_tile.is_ok());
        assert_eq!(
            game.round.tile_claimed,
            Some(RoundTileClaimed {
                by: None,
                from: game.players.0[0].clone(),
                id: Tile::id_from_summary("二萬"),
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
        let mut game = Game::new(None);

        game.table.draw_wall.position_tiles(None);

        assert_eq!(game.players.len(), 4);

        for player in game.players.iter() {
            let hand = game.table.hands.get(player);
            assert_eq!(hand.unwrap().len(), 0);
            assert_eq!(game.score.get(player), Some(&0));
        }

        assert_eq!(game.table.draw_wall.len(), DEFAULT_DECK.0.len());
        assert_eq!(game.table.board.len(), 0);
    }
}
