use std::str::FromStr;

use uuid::Uuid;

use crate::{
    deck::DEFAULT_DECK,
    hand::HandPossibleMeld,
    round::{Round, RoundTileClaimed},
    table::BonusTiles,
    Board, Deck, Dragon, DragonTile, DrawWall, Flower, FlowerTile, Game, GamePhase, Hand, HandTile,
    Hands, Season, SeasonTile, Suit, SuitTile, Tile, TileId, Wind, WindTile,
};

pub fn print_game_tile(tile: &Tile) -> String {
    let mut result = String::new();

    match tile {
        Tile::Dragon(tile) => {
            let dragon_letter = match tile.value {
                Dragon::Red => '中',
                Dragon::Green => '發',
                Dragon::White => '白',
            };
            result.push_str(&dragon_letter.to_string());
        }
        Tile::Wind(tile) => {
            let wind_letter = match tile.value {
                Wind::East => '東',
                Wind::North => '北',
                Wind::South => '南',
                Wind::West => '西',
            };
            result.push_str(&wind_letter.to_string());
        }
        Tile::Flower(tile) => {
            let flower_letter = match tile.value {
                Flower::Plum => '梅',
                Flower::Orchid => '蘭',
                Flower::Chrysanthemum => '菊',
                Flower::Bamboo => '竹',
            };
            result.push_str(&flower_letter.to_string());
        }
        Tile::Season(tile) => {
            let season_letter = match tile.value {
                Season::Spring => '春',
                Season::Summer => '夏',
                Season::Autumn => '秋',
                Season::Winter => '冬',
            };
            result.push_str(&season_letter.to_string());
        }
        Tile::Suit(tile) => {
            let value_str = match tile.value {
                1 => '一',
                2 => '二',
                3 => '三',
                4 => '四',
                5 => '五',
                6 => '六',
                7 => '七',
                8 => '八',
                9 => '九',
                _ => panic!("Invalid value"),
            };
            let suit_letter = match tile.suit {
                Suit::Bamboo => '索',
                Suit::Dots => '筒',
                Suit::Characters => '萬',
            };
            result.push_str(&format!("{:}{:}", value_str, suit_letter));
        }
    }

    result
}

impl FromStr for GamePhase {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Beginning" => Ok(Self::Beginning),
            "Deciding Dealer" => Ok(Self::DecidingDealer),
            "End" => Ok(Self::End),
            "Initial Draw" => Ok(Self::InitialDraw),
            "Playing" => Ok(Self::Playing),
            _ => Err(()),
        }
    }
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
    pub fn get_summary(&self) -> String {
        let mut result = String::new();

        for (pos, player) in self.players.iter().enumerate() {
            if self.table.hands.get(player).is_empty() {
                continue;
            }
            result.push('\n');
            result.push_str(&format!("- P{}: ", pos + 1));
            let hand = self.table.hands.get(player);

            result.push_str(&hand.to_summary_full());
        }

        if !self.table.draw_wall.is_empty() {
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
        }

        if !self.table.board.0.is_empty() {
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

        result.push_str("\nConsecutive: ");
        result.push_str(&format!("{}", self.round.consecutive_same_seats));
        if let Some(tile) = self.round.tile_claimed.clone() {
            result.push_str(", Discarded: ");
            result.push_str(&print_game_tile(DEFAULT_DECK.0.get(&tile.id).unwrap()));
            if let Some(by) = tile.by {
                result.push_str("(P");
                result.push_str(&(by.parse::<usize>().unwrap() + 1).to_string());
                result.push(')');
            }
        }
        if let Some(tile) = self.round.wall_tile_drawn {
            result.push_str(", Drawn: ");
            result.push_str(&print_game_tile(DEFAULT_DECK.0.get(&tile).unwrap()));
        }

        result.trim().to_string()
    }

    pub fn from_summary(summary: &str) -> Self {
        let mut game = Self::new(None);
        let mut lines = summary.trim().lines();

        let mut line = lines.next().unwrap().trim();

        for (idx, player) in game.players.iter().enumerate() {
            let prefix = format!("- P{}: ", idx + 1);
            if !line.starts_with(&prefix) {
                game.table.hands.update_player_hand(player, "");
                continue;
            }
            let hand = Hand::from_summary(&line[5..]);
            game.table.hands.0.insert(player.clone(), hand);
            game.table.bonus_tiles.set_from_summary(player, &line[5..]);

            line = lines.next().unwrap().trim();
        }

        if let Some(wall_line) = line.strip_prefix("Wall:") {
            if !wall_line.trim().starts_with("Random") {
                let wall_line = wall_line.trim().replace("...", "");
                if wall_line.is_empty() {
                    game.table.draw_wall.0.clear();
                } else {
                    game.table.draw_wall.replace_tail_summary(&wall_line);
                }
            }
            line = lines.next().unwrap();
        } else {
            game.table.draw_wall.0.clear();
        }

        if line.starts_with("Board: ") {
            let board_line = line[7..].replace("...", "");
            game.table.board.push_by_summary(&board_line);
            line = lines.next().unwrap_or("");
        }

        line.trim().split(", ").for_each(|fragment| {
            if fragment.starts_with("Turn: ") {
                let turn_player = fragment[7..].parse::<usize>().unwrap();
                game.round.player_index = turn_player - 1;
            } else if fragment.starts_with("Dealer: ") {
                let dealer_player = fragment[9..].parse::<usize>().unwrap();
                game.round.dealer_player_index = dealer_player - 1;
            } else if let Some(round_num) = fragment.strip_prefix("Round: ") {
                let round_index = round_num.parse::<u32>().unwrap();
                game.round.round_index = round_index - 1;
            } else if let Some(wind) = fragment.strip_prefix("Wind: ") {
                game.round.wind = Wind::from_str(wind.trim()).unwrap();
            } else if let Some(phase) = fragment.strip_prefix("Phase: ") {
                game.phase = GamePhase::from_str(phase.trim()).unwrap();
            }
        });

        line = lines.next().unwrap_or("");

        line.trim().split(", ").for_each(|fragment| {
            if let Some(count) = fragment.strip_prefix("Consecutive: ") {
                let consecutive = count.parse::<usize>().unwrap();
                game.round.consecutive_same_seats = consecutive;
            } else if let Some(tile) = fragment.strip_prefix("Drawn: ") {
                let tile_id = Tile::id_from_summary(tile.trim());
                game.round.wall_tile_drawn = Some(tile_id);
            } else if fragment.starts_with("First East: ") {
                let player_num = fragment[13..].parse::<usize>().unwrap();
                game.round.east_player_index = player_num - 1;
            } else if let Some(tile) = fragment.strip_prefix("Discarded: ") {
                let (from, by) = if tile.contains('(') {
                    let mut parts = tile.split('(');
                    let from = parts.next().unwrap().trim();
                    let by = parts
                        .next()
                        .unwrap()
                        .trim()
                        .strip_prefix('P')
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                        - 1;
                    (from, Some(by.to_string()))
                } else {
                    (tile, None)
                };
                game.round.tile_claimed = Some(RoundTileClaimed {
                    by,
                    from: game.players.0[game.round.player_index].clone(),
                    id: Tile::id_from_summary(from),
                });
            }
        });

        game
    }
}

impl Tile {
    pub fn from_summary(summary: &str) -> Self {
        let first_char = summary.chars().nth(0).unwrap();
        let tile = match first_char {
            '一' | '二' | '三' | '四' | '五' | '六' | '七' | '八' | '九' => {
                let value = match first_char {
                    '一' => 1,
                    '二' => 2,
                    '三' => 3,
                    '四' => 4,
                    '五' => 5,
                    '六' => 6,
                    '七' => 7,
                    '八' => 8,
                    '九' => 9,
                    _ => panic!("Invalid value"),
                };
                let suit = match summary.chars().nth(1).unwrap() {
                    '索' => Suit::Bamboo,
                    '筒' => Suit::Dots,
                    '萬' => Suit::Characters,
                    _ => panic!("Invalid suit"),
                };
                Self::Suit(SuitTile { id: 0, value, suit })
            }
            '東' | '南' | '西' | '北' => {
                let value = match first_char {
                    '東' => Wind::East,
                    '北' => Wind::North,
                    '南' => Wind::South,
                    '西' => Wind::West,
                    _ => panic!("Invalid wind"),
                };
                Self::Wind(WindTile { id: 0, value })
            }
            '中' | '發' | '白' => {
                let value = match first_char {
                    '中' => Dragon::Red,
                    '發' => Dragon::Green,
                    '白' => Dragon::White,
                    _ => panic!("Invalid dragon"),
                };
                Self::Dragon(DragonTile { id: 0, value })
            }
            '梅' | '蘭' | '菊' | '竹' => {
                let value = match first_char {
                    '梅' => Flower::Plum,
                    '蘭' => Flower::Orchid,
                    '菊' => Flower::Chrysanthemum,
                    '竹' => Flower::Bamboo,
                    _ => panic!("Invalid flower"),
                };
                Self::Flower(FlowerTile { id: 0, value })
            }
            '春' | '夏' | '秋' | '冬' => {
                let value = match first_char {
                    '春' => Season::Spring,
                    '夏' => Season::Summer,
                    '秋' => Season::Autumn,
                    '冬' => Season::Winter,
                    _ => panic!("Invalid season"),
                };
                Self::Season(SeasonTile { id: 0, value })
            }
            _ => panic!("Invalid summary: {summary}"),
        };
        Deck::find_tile_without_id(tile)
    }

    pub fn summary_from_ids(ids: &[TileId]) -> String {
        ids.iter()
            .map(|id| print_game_tile(DEFAULT_DECK.get_sure(*id)))
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn id_from_summary(summary: &str) -> TileId {
        Self::from_summary(summary).get_id()
    }

    pub fn ids_from_summary(summary: &str) -> Vec<TileId> {
        summary
            .split(',')
            .filter(|tile| !tile.is_empty())
            .map(Self::id_from_summary)
            .collect()
    }
}

impl HandPossibleMeld {
    pub fn from_summary(summary: &str) -> Self {
        let summary_parts = summary.split(' ').collect::<Vec<&str>>();

        match summary_parts.len() {
            2 => Self {
                is_mahjong: summary_parts[1] == "YES",
                tiles: Hand::from_summary(summary_parts[0])
                    .list
                    .iter()
                    .map(|t| t.id)
                    .collect(),
            },
            _ => panic!("Invalid summary: {}", summary),
        }
    }

    pub fn from_summaries(summary: &[&str]) -> Vec<Self> {
        summary.iter().map(|s| Self::from_summary(s)).collect()
    }

    pub fn to_summary(&self) -> String {
        let result = Hand::from_ids(&self.tiles);
        let mut result_summary = result.to_summary();

        if self.is_mahjong {
            result_summary.push_str(" YES");
        } else {
            result_summary.push_str(" NO");
        }
        result_summary
    }
}

impl HandTile {
    pub fn from_test_summary(summary: &str) -> Self {
        Self::from_tile(&Tile::from_summary(summary))
    }
}

impl Hand {
    pub fn from_summary(summary: &str) -> Self {
        Self::new(
            summary
                .split(' ')
                .filter(|tile_set| !tile_set.is_empty())
                .enumerate()
                .flat_map(|(idx, tile_set)| {
                    let set_id = if idx == 0 {
                        None
                    } else {
                        Some(Uuid::new_v4().to_string())
                    };
                    let (concealed, parsed_set) =
                        if let Some(tile_set_plain) = tile_set.strip_prefix('*') {
                            (false, tile_set_plain.to_string())
                        } else {
                            (true, tile_set.to_string())
                        };

                    parsed_set
                        .split(',')
                        .filter(|tile| !tile.is_empty())
                        .filter_map(|tile| {
                            let tile = Tile::from_summary(tile);
                            if tile.is_bonus() {
                                return None;
                            }
                            let mut hand_tile = HandTile::from_tile(&tile);
                            hand_tile.set_id.clone_from(&set_id);
                            hand_tile.concealed = concealed;
                            Some(hand_tile)
                        })
                        .collect::<Vec<HandTile>>()
                })
                .collect(),
        )
    }

    pub fn to_summary(&self) -> String {
        let mut sets_parsed = self
            .list
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
            if tiles.len() > 1 && !tiles[0].concealed {
                result.push('*');
            }

            result.push_str(&Self::from_ref_vec(tiles).to_summary());
        }

        result
    }
}

impl Hands {
    pub fn update_player_hand(&mut self, player_id: impl AsRef<str>, summary: &str) -> &mut Self {
        self.0
            .insert(player_id.as_ref().to_string(), Hand::from_summary(summary));
        self
    }
    pub fn update_players_hands(&mut self, summaries: &[&str]) -> &mut Self {
        summaries.iter().enumerate().for_each(|(idx, summary)| {
            self.update_player_hand(idx.to_string(), summary);
        });
        self
    }
}

impl Board {
    pub fn from_summary(summary: &str) -> Self {
        let mut board = Self::default();
        board.push_by_summary(summary);
        board
    }

    pub fn push_by_summary(&mut self, summary: &str) {
        summary
            .split(',')
            .filter(|tile| !tile.is_empty())
            .for_each(|tile| {
                self.0.push(Tile::id_from_summary(tile));
            });
    }

    pub fn to_summary(&self) -> String {
        self.0
            .iter()
            .map(|tile| print_game_tile(DEFAULT_DECK.get_sure(*tile)))
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl DrawWall {
    pub fn from_summary(summary: &str) -> Self {
        let tiles = summary
            .split(',')
            .filter(|tile| !tile.is_empty())
            .map(Tile::id_from_summary)
            .collect();

        Self(tiles)
    }

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

impl BonusTiles {
    pub fn set_from_summary(&mut self, player_id: &str, summary: &str) {
        self.0.insert(
            player_id.to_string(),
            summary
                .trim()
                .replace(' ', ",")
                .replace('*', "")
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| Tile::id_from_summary(s.trim().replace(' ', ",").as_ref()))
                .filter(|tile_id| DEFAULT_DECK.get_sure(*tile_id).is_bonus())
                .collect(),
        );
    }
}

impl Round {
    pub fn from_summary(summary: &str) -> Self {
        let game = Game::from_summary(summary);

        game.round
    }
}
