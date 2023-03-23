use mahjong_core::{
    Board, Deck, Dragon, Flower, Game, Hand, Player, Score, Season, Suit, Tile, Wind,
};
use service_contracts::GameSummary;

fn format_to_emoji(tile: &Tile) -> String {
    match tile {
        Tile::Suit(tile) => match tile.suit {
            Suit::Bamboo => format!("🎋{}", tile.value),
            Suit::Characters => format!("✨{}", tile.value),
            Suit::Dots => format!("💠{}", tile.value),
        },
        Tile::Wind(tile) => match tile.value {
            Wind::East => "🍃EA".to_string(),
            Wind::North => "🍃NO".to_string(),
            Wind::South => "🍃SO".to_string(),
            Wind::West => "🍃WE".to_string(),
        },
        Tile::Dragon(tile) => match tile.value {
            Dragon::Green => "🐉GR".to_string(),
            Dragon::Red => "🐉RE".to_string(),
            Dragon::White => "🐉WH".to_string(),
        },
        Tile::Flower(tile) => match tile.value {
            Flower::Bamboo => "💮BA".to_string(),
            Flower::Chrysanthemum => "💮CH".to_string(),
            Flower::Orchid => "💮OR".to_string(),
            Flower::Plum => "💮PL".to_string(),
        },
        Tile::Season(tile) => match tile.value {
            Season::Autumn => "🌞AU".to_string(),
            Season::Spring => "🌞SP".to_string(),
            Season::Summer => "🌞SU".to_string(),
            Season::Winter => "🌞WI".to_string(),
        },
    }
}

pub fn get_draw_wall(game: &Game) -> String {
    game.table
        .draw_wall
        .iter()
        .map(|tile_id| {
            let tile = game.deck.0.get(tile_id).unwrap();
            let tile_str = format_to_emoji(tile);

            format!("[{}]", tile_str)
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn get_board(board: &Board, deck: &Deck) -> String {
    board
        .iter()
        .map(|tile_id| {
            let tile = deck.0.get(tile_id).unwrap();
            let tile_str = format_to_emoji(tile);

            format!("[{}]", tile_str)
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn get_user_hand_str(game: &GameSummary) -> Vec<String> {
    let mut lines = vec![];

    lines.push("- Hand:".to_string());

    format_hand(&game.hand, &game.deck)
        .iter()
        .for_each(|line| lines.push(line.to_string()));

    lines
}

pub fn format_hand(hand: &Hand, deck: &Deck) -> Vec<String> {
    let mut lines = vec![];

    let line = hand
        .0
        .iter()
        .filter(|tile| tile.set_id.is_none())
        .enumerate()
        .map(|(idx, hand_tile)| {
            let tile = deck.0.get(&hand_tile.id).unwrap();
            let tile_str = format_to_emoji(tile);
            let idx_formatted = format!("{:0>2}", idx);
            format!("[{idx_formatted}]({tile_str})",)
        })
        .collect::<Vec<String>>()
        .join("  ");

    // This can be empty when printing other players
    if !line.is_empty() {
        lines.push(line);
    }

    let melds = hand.get_melds();

    melds.melds.iter().for_each(|(_, tiles)| {
        let mut full_tiles = tiles
            .iter()
            .map(|tile| deck.0.get(&tile.id).unwrap())
            .collect::<Vec<&Tile>>();
        let is_concealed = tiles.iter().any(|tile| tile.concealed);
        full_tiles.sort_by(|a, b| a.cmp_custom(b));
        let meld_str = full_tiles
            .iter()
            .map(|tile| format!("({})", format_to_emoji(tile)))
            .collect::<Vec<String>>()
            .join(" ");

        lines.push(format!(
            "- Meld {}: {}",
            {
                if is_concealed {
                    "🔒"
                } else {
                    "👁"
                }
            },
            meld_str
        ));
    });

    lines
}

pub fn format_player(
    player: &Player,
    current_player: &Player,
    hand: Option<&Hand>,
    score: &Score,
    player_tiles: Option<usize>,
) -> String {
    format!(
        "{}: {}{} [{}] <{}>",
        player.name,
        if hand.is_some() {
            hand.unwrap().0.len().to_string()
        } else if player_tiles.is_some() {
            player_tiles.unwrap().to_string()
        } else {
            "?".to_string()
        },
        if player.id == current_player.id {
            " *"
        } else {
            ""
        },
        player.id,
        score.get(&player.id).unwrap()
    )
}

pub fn get_admin_hands_str(game: &Game) -> Vec<String> {
    let mut lines = vec![];
    let current_player = game.get_current_player();
    let mut possible_melds = game.get_possible_melds_by_discard();

    for player in game.players.iter() {
        let hand = game.table.hands.get(&player.id).unwrap();

        lines.push("".to_string());
        lines.push(format_player(
            player,
            current_player,
            Some(hand),
            &game.score,
            None,
        ));

        format_hand(hand, &game.deck).iter().for_each(|line| {
            lines.push(line.to_string());
        });

        possible_melds.iter_mut().for_each(|meld| {
            if meld.player_id == player.id {
                meld.sort_tiles(&game.deck);

                let meld_str = meld
                    .tiles
                    .iter()
                    .map(|tile_id| {
                        let tile = &game.deck.0[tile_id];
                        let tile_index =
                            hand.0.iter().position(|hand_tile| hand_tile.id == *tile_id);

                        format!(
                            "[{}]({})",
                            if tile_index.is_some() {
                                tile_index.unwrap().to_string()
                            } else {
                                '_'.to_string()
                            },
                            format_to_emoji(tile)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(" ");

                lines.push(format!(
                    "- Possible Meld:  {} ({})",
                    meld_str,
                    if meld.discard_tile.is_some() {
                        let tile = game.deck.0.get(&meld.discard_tile.unwrap()).unwrap();
                        let emoji = format_to_emoji(tile);
                        let current_player_hand = game.table.hands.get(&current_player.id).unwrap();
                        let index = current_player_hand
                            .0
                            .iter()
                            .position(|hand_tile| hand_tile.id == tile.get_id())
                            .unwrap();

                        format!("Discard: [{index}]({emoji})")
                    } else {
                        "No discard".to_string()
                    }
                ));
            }
        });
    }

    lines
}
