#[cfg(test)]
mod test {
    use crate::{DrawWall, Game};
    use pretty_assertions::assert_eq;

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
        game.table.hands.update_player_hand("0", "1D");
        game.table.hands.update_player_hand("1", "");
        game.table.hands.update_player_hand("2", "");
        game.table.hands.update_player_hand("3", "");
        game.version = "bd760511-27d9-4c32-a1bb-8d2795bc3c42".to_string();

        let game_str = serde_json::to_string_pretty(&game).unwrap();

        assert_eq!(game_str.trim(), GAME_EXPECTED.trim())
    }

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
    "dealer_player_index": 0,
    "player_index": 0,
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
    }
  },
  "version": "bd760511-27d9-4c32-a1bb-8d2795bc3c42",
  "style": "HongKong"
}
"#;
}
