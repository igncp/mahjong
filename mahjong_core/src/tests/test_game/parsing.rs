#[cfg(test)]
mod test {
    use crate::{round::RoundTileClaimed, DrawWall, Game, Tile, Wind};
    use pretty_assertions::assert_eq;

    const GAME_EXPECTED: &str = r#"
{
  "id": "game_id",
  "name": "game_name",
  "phase": "Beginning",
  "players": [
    "0",
    "1",
    "2",
    "3"
  ],
  "round": {
    "consecutive_same_seats": 0,
    "dealer_player_index": 0,
    "player_index": 0,
    "east_player_index": 0,
    "round_index": 0,
    "tile_claimed": null,
    "wall_tile_drawn": null,
    "wind": "East"
  },
  "score": {
    "3": 0,
    "2": 0,
    "0": 0,
    "1": 0
  },
  "table": {
    "board": [],
    "draw_wall": [
      121,
      12,
      65,
      62,
      113,
      78,
      119,
      63,
      105,
      88,
      142,
      85,
      107,
      21,
      109,
      81,
      13,
      66,
      71,
      106,
      99,
      6,
      98,
      73,
      108,
      83,
      25,
      61,
      86,
      77,
      35,
      23,
      133,
      49,
      92,
      47,
      94,
      50,
      100,
      84,
      31,
      5,
      129,
      19,
      14,
      33,
      96,
      117,
      139,
      27,
      80,
      128,
      74,
      53,
      140,
      30,
      67,
      125,
      43,
      48,
      115,
      137,
      97,
      90,
      51,
      114,
      17,
      82,
      59,
      79,
      75,
      91,
      55,
      37,
      135,
      60,
      123,
      29,
      143,
      10,
      41,
      95,
      57,
      26,
      131,
      116,
      4,
      101,
      34,
      32,
      132,
      118
    ],
    "hands": {
      "3": [],
      "2": [],
      "0": [
        {
          "concealed": true,
          "id": 119,
          "set_id": null
        }
      ],
      "1": []
    },
    "bonus_tiles": {
      "0": [
        2
      ]
    }
  },
  "version": "bd760511-27d9-4c32-a1bb-8d2795bc3c42",
  "style": "HongKong"
}
"#;

    #[test]
    fn test_game_parsing() {
        let mut game = Game::new(None);

        game.table.draw_wall = DrawWall(vec![
            121, 12, 65, 62, 113, 78, 119, 63, 105, 88, 142, 85, 107, 21, 109, 81, 13, 66, 71, 106,
            99, 6, 98, 73, 108, 83, 25, 61, 86, 77, 35, 23, 133, 49, 92, 47, 94, 50, 100, 84, 31,
            5, 129, 19, 14, 33, 96, 117, 139, 27, 80, 128, 74, 53, 140, 30, 67, 125, 43, 48, 115,
            137, 97, 90, 51, 114, 17, 82, 59, 79, 75, 91, 55, 37, 135, 60, 123, 29, 143, 10, 41,
            95, 57, 26, 131, 116, 4, 101, 34, 32, 132, 118,
        ]);
        game.table.hands.update_player_hand("0", "一筒");
        game.table.hands.update_player_hand("1", "");
        game.table.hands.update_player_hand("2", "");
        game.table.hands.update_player_hand("3", "");

        game.table.bonus_tiles.set_from_summary("0", "蘭");

        game.version = "bd760511-27d9-4c32-a1bb-8d2795bc3c42".to_string();

        let game_str = serde_json::to_string_pretty(&game).unwrap();

        assert_eq!(game_str.trim(), GAME_EXPECTED.trim());

        let game_deserialized: Game = serde_json::from_str(GAME_EXPECTED).unwrap();

        assert_eq!(
            game.table.draw_wall.0.len(),
            game_deserialized.table.draw_wall.0.len()
        );
    }

    #[test]
    fn test_game_print_summary() {
        let mut game = Game::new(None);

        game.table.hands.update_players_hands(&[
            "一索,七筒 二萬,二萬,二萬",
            "二筒",
            "三筒",
            "五筒,四筒",
        ]);

        game.table
            .draw_wall
            .replace_tail_summary("五筒,四筒,三筒,四筒");
        game.table.board.push_by_summary("六筒,四筒");
        game.round.consecutive_same_seats = 2;
        game.round.wall_tile_drawn = Some(Tile::from_summary("一索").get_id());
        game.round.tile_claimed = Some(RoundTileClaimed {
            by: Some("0".to_string()),
            from: "0".to_string(),
            id: Tile::from_summary("七筒").get_id(),
        });

        game.start();

        assert_eq!(
            game.get_summary().trim(),
            r#"
- P1: 一索,七筒 二萬,二萬,二萬
- P2: 二筒
- P3: 三筒
- P4: 五筒,四筒
Wall: 五筒,四筒,三筒...
Board: 四筒,六筒
Turn: P1, Dealer: P1, Round: 1, Wind: East, Phase: Playing
Consecutive: 2, Discarded: 七筒(P1), Drawn: 一索
"#
            .trim()
        );
    }

    #[test]
    fn test_game_from_summary() {
        let game = Game::from_summary(
            r#"
- P1: 一索 二萬,二萬,二萬
- P2: 二筒
- P3: 三筒,蘭
- P4: 四筒,五筒
Wall: 五筒,四筒,三筒...
Board: 五筒,四筒...
Turn: P3, Dealer: P4, Round: 2, Wind: East, Phase: Playing
Consecutive: 1, Drawn: 三筒, Discarded: 五筒
"#,
        );

        assert_eq!(
            game.table.draw_wall.0.last().unwrap().clone(),
            Tile::from_summary("五筒").get_id()
        );

        assert_eq!(game.table.hands.get_player_hand_len("0"), 4);
        assert_eq!(game.table.hands.get_player_hand_len("1"), 1);
        assert_eq!(game.table.hands.get_player_hand_len("2"), 1);
        assert_eq!(game.table.hands.get_player_hand_len("3"), 2);
        assert_eq!(game.table.bonus_tiles.0.get("2").unwrap().len(), 1);
        assert_eq!(game.round.player_index, 2);
        assert_eq!(game.round.dealer_player_index, 3);
        assert_eq!(game.round.round_index, 1);
        assert_eq!(game.round.consecutive_same_seats, 1);
        assert_eq!(game.round.wind, Wind::East);
        assert_eq!(
            game.round.tile_claimed.unwrap().id,
            Tile::from_summary("五筒").get_id()
        );
        assert_eq!(
            game.round.wall_tile_drawn,
            Some(Tile::id_from_summary("三筒"))
        );
    }
}
