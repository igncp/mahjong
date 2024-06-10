use crate::{
    deck::DEFAULT_DECK, Board, Deck, Dragon, DragonTile, DrawWall, Flower, FlowerTile, Game, Hand,
    HandTile, Hands, Season, SeasonTile, Suit, SuitTile, Tile, TileId, Wind, WindTile,
};

pub fn print_game_tile(tile: &Tile) -> String {
    let mut result = String::new();

    match tile {
        Tile::Dragon(tile) => {
            let dragon_letter = match tile.value {
                Dragon::Green => 'G',
                Dragon::Red => 'R',
                Dragon::White => 'W',
            };
            result.push_str(&format!("d{:}", dragon_letter));
        }
        Tile::Wind(tile) => {
            let wind_letter = match tile.value {
                Wind::East => 'E',
                Wind::North => 'N',
                Wind::South => 'S',
                Wind::West => 'W',
            };
            result.push_str(&format!("w{:}", wind_letter));
        }
        Tile::Flower(tile) => {
            let flower_letter = match tile.value {
                Flower::Bamboo => 'B',
                Flower::Chrysanthemum => 'C',
                Flower::Orchid => 'O',
                Flower::Plum => 'P',
            };
            result.push_str(&format!("f{:}", flower_letter));
        }
        Tile::Season(tile) => {
            let season_letter = match tile.value {
                Season::Spring => 'S',
                Season::Summer => 'M',
                Season::Autumn => 'A',
                Season::Winter => 'W',
            };
            result.push_str(&format!("s{:}", season_letter));
        }
        Tile::Suit(tile) => {
            let suit_letter = match tile.suit {
                Suit::Bamboo => 'B',
                Suit::Characters => 'C',
                Suit::Dots => 'D',
            };
            result.push_str(&format!("{:01}{:}", tile.value, suit_letter));
        }
    }

    result
}

impl Wind {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "East" => Ok(Self::East),
            "North" => Ok(Self::North),
            "South" => Ok(Self::South),
            "West" => Ok(Self::West),
            _ => Err(format!("Invalid wind: {}", s)),
        }
    }
}

impl Game {
    pub fn print_summary(&self) -> String {
        let mut result = String::new();

        for (pos, player) in self.players.iter().enumerate() {
            result.push('\n');
            result.push_str(&format!("- P{}: ", pos + 1));
            let hand = self.table.hands.get(player);

            result.push_str(&hand.to_summary_full());
        }

        result.push_str("\nWall: ");
        result.push_str(
            &self
                .table
                .draw_wall
                .0
                .iter()
                .map(|tile| print_game_tile(DEFAULT_DECK.0.get(tile).unwrap()))
                .rev()
                .take(3)
                .collect::<Vec<String>>()
                .join(","),
        );
        if self.table.draw_wall.len() > 3 {
            result.push_str("...");
        }

        result.push_str("\nBoard: ");
        let mut parsed_board = self
            .table
            .board
            .0
            .iter()
            .map(|tile| print_game_tile(DEFAULT_DECK.0.get(tile).unwrap()))
            .collect::<Vec<String>>();
        parsed_board.reverse();
        result.push_str(&parsed_board[0..2].join(","));
        if parsed_board.len() > 2 {
            result.push_str("...");
        }

        result.push_str("\nTurn: ");
        result.push_str(&format!("P{}", self.round.player_index + 1));
        result.push_str(", Dealer: ");
        result.push_str(&format!("P{}", self.round.dealer_player_index + 1));
        result.push_str(", Round: ");
        result.push_str(&format!("{}", self.round.round_index + 1));
        result.push_str(", Wind: ");
        result.push_str(&format!("{:?}", self.round.wind));
        result.push_str(", Phase: ");
        result.push_str(&format!("{:?}", self.phase));

        result
    }

    pub fn from_summary(summary: &str) -> Self {
        let mut game = Self::new(None);
        let mut lines = summary.trim().lines();

        for player in game.players.iter() {
            let line = lines.next().unwrap();
            let hand = Hand::from_summary(&line[5..]);
            game.table.hands.0.insert(player.clone(), hand);
        }

        let wall_line = lines.next().unwrap()[6..].replace("...", "");
        game.table.draw_wall.replace_tail_summary(&wall_line);

        let board_line = lines.next().unwrap()[7..].replace("...", "");
        game.table.board.push_by_summary(&board_line);

        let round_line = lines.next().unwrap();
        let round_parts = round_line.split(", ").collect::<Vec<&str>>();
        let turn_player = round_parts[0].split(": ").nth(1).unwrap();
        game.round.player_index = turn_player[1..].parse::<usize>().unwrap() - 1;
        let dealer_player = round_parts[1].split(": ").nth(1).unwrap();
        game.round.dealer_player_index = dealer_player[1..].parse::<usize>().unwrap() - 1;
        let round_index = round_parts[2].split(": ").nth(1).unwrap();
        game.round.round_index = round_index.parse::<u32>().unwrap() - 1;
        let wind = round_parts[3].split(": ").nth(1).unwrap();
        game.round.wind = Wind::from_str(wind).unwrap();

        game
    }
}

impl Tile {
    pub fn from_summary(summary: &str) -> Self {
        let tile = match summary.len() {
            2 => match summary.chars().nth(0).unwrap() {
                '1'..='9' => {
                    let value = summary[0..1].parse::<u32>().unwrap();
                    let suit = match summary.chars().nth(1).unwrap() {
                        'B' => Suit::Bamboo,
                        'C' => Suit::Characters,
                        'D' => Suit::Dots,
                        _ => panic!("Invalid suit"),
                    };
                    Self::Suit(SuitTile { id: 0, value, suit })
                }
                'w' => {
                    let value = match summary.chars().nth(1).unwrap() {
                        'E' => Wind::East,
                        'N' => Wind::North,
                        'S' => Wind::South,
                        'W' => Wind::West,
                        _ => panic!("Invalid wind"),
                    };
                    Self::Wind(WindTile { id: 0, value })
                }
                'd' => {
                    let value = match summary.chars().nth(1).unwrap() {
                        'G' => Dragon::Green,
                        'R' => Dragon::Red,
                        'W' => Dragon::White,
                        _ => panic!("Invalid dragon"),
                    };
                    Self::Dragon(DragonTile { id: 0, value })
                }
                'f' => {
                    let value = match summary.chars().nth(1).unwrap() {
                        'B' => Flower::Bamboo,
                        'C' => Flower::Chrysanthemum,
                        'O' => Flower::Orchid,
                        'P' => Flower::Plum,
                        _ => panic!("Invalid flower"),
                    };
                    Self::Flower(FlowerTile { id: 0, value })
                }
                's' => {
                    let value = match summary.chars().nth(1).unwrap() {
                        'S' => Season::Spring,
                        'M' => Season::Summer,
                        'A' => Season::Autumn,
                        'W' => Season::Winter,
                        _ => panic!("Invalid season"),
                    };
                    Self::Season(SeasonTile { id: 0, value })
                }
                _ => panic!("Invalid summary"),
            },
            _ => panic!("Invalid summary"),
        };
        Deck::find_tile_without_id(tile)
    }

    pub fn id_from_summary(summary: &str) -> TileId {
        Self::from_summary(summary).get_id()
    }
}

impl HandTile {
    pub fn from_test_summary(summary: &str) -> Self {
        Self::from_tile(&Tile::from_summary(summary))
    }
}

impl Hand {
    pub fn from_summary(summary: &str) -> Self {
        Self(
            summary
                .split(' ')
                .filter(|tile_set| !tile_set.is_empty())
                .enumerate()
                .flat_map(|(idx, tile_set)| {
                    tile_set
                        .split(',')
                        .filter(|tile| !tile.is_empty())
                        .map(|tile| {
                            let set_id = if idx == 0 {
                                None
                            } else {
                                Some(idx.to_string())
                            };
                            let mut hand_tile = HandTile::from_test_summary(tile);
                            hand_tile.set_id = set_id;
                            hand_tile
                        })
                        .collect::<Vec<HandTile>>()
                })
                .collect(),
        )
    }

    pub fn to_summary(&self) -> String {
        let mut sets_parsed = self
            .0
            .iter()
            .map(|tile| print_game_tile(DEFAULT_DECK.get_sure(tile.id)))
            .collect::<Vec<String>>();
        sets_parsed.sort();
        sets_parsed.join(",")
    }

    pub fn to_summary_full(&self) -> String {
        let mut result = String::new();
        let sets_groups = self.get_sets_groups();

        if let Some(tiles) = sets_groups.get(&None) {
            result.push_str(&Self::from_ref_vec(tiles).to_summary());
        }

        for (_, tiles) in sets_groups.iter().filter(|(set_id, _)| set_id.is_some()) {
            result.push(' ');
            result.push_str(&Self::from_ref_vec(tiles).to_summary());
        }

        result
    }
}

impl Hands {
    pub fn update_player_hand(&mut self, player_id: impl AsRef<str>, summary: &str) {
        self.0
            .insert(player_id.as_ref().to_string(), Hand::from_summary(summary));
    }
}

impl Board {
    pub fn push_by_summary(&mut self, summary: &str) {
        summary
            .split(',')
            .filter(|tile| !tile.is_empty())
            .for_each(|tile| {
                self.0.push(Tile::id_from_summary(tile));
            });
    }
}

impl DrawWall {
    pub fn replace_tail_summary(&mut self, summary: &str) {
        let len = self.len();
        summary
            .split(',')
            .filter(|tile| !tile.is_empty())
            .take(len)
            .enumerate()
            .for_each(|(tail_idx, summary_item)| {
                self.0[len - tail_idx - 1] = Tile::id_from_summary(summary_item);
            });
    }
}
