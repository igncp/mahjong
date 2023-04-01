use mahjong_core::{ui::format_to_emoji, Board, Deck, Game, Hand, PlayerId, Score, Tile};
use service_contracts::{GameSummary, ServiceGame};

pub fn get_draw_wall(game: &Game, show_index: bool) -> String {
    game.table
        .draw_wall
        .iter()
        .enumerate()
        .map(|(index, tile_id)| {
            let tile = game.deck.0.get(tile_id).unwrap();
            let tile_str = format_to_emoji(tile);

            if show_index {
                format!("[{:0>2}]({})", index, tile_str)
            } else {
                format!("[{}]", tile_str)
            }
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
    let mut melds_str = vec![];

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

        melds_str.push(format!(
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

    melds_str.sort();

    melds_str
        .iter()
        .for_each(|line| lines.push(line.to_string()));

    lines
}

pub fn format_player(
    player: &PlayerId,
    current_player: &PlayerId,
    hand: Option<&Hand>,
    score: &Score,
    player_tiles: Option<usize>,
) -> String {
    format!(
        "{}{} [{}] <{}>",
        if hand.is_some() {
            hand.unwrap().0.len().to_string()
        } else if player_tiles.is_some() {
            player_tiles.unwrap().to_string()
        } else {
            "?".to_string()
        },
        if player == current_player { " *" } else { "" },
        player,
        score.get(player).unwrap()
    )
}

pub fn get_admin_hands_str(game: &ServiceGame) -> Vec<String> {
    let mut lines = vec![];
    let current_player = game.game.get_current_player();
    let mut possible_melds = game.game.get_possible_melds_by_discard();

    for player in game.game.players.iter() {
        let hand = game.game.table.hands.get(player).unwrap();

        lines.push("".to_string());
        let player_line =
            format_player(player, &current_player, Some(hand), &game.game.score, None);
        let service_player = game.players.get(player).unwrap();
        lines.push(format!(
            "{}{}: {}",
            if service_player.is_ai { "[AI] " } else { "" },
            service_player.name,
            player_line
        ));

        format_hand(hand, &game.game.deck).iter().for_each(|line| {
            lines.push(line.to_string());
        });

        possible_melds.iter_mut().for_each(|meld| {
            if &meld.player_id == player {
                meld.sort_tiles(&game.game.deck);

                let meld_str = meld
                    .tiles
                    .iter()
                    .map(|tile_id| {
                        let tile = &game.game.deck.0[tile_id];
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
                        let tile = game.game.deck.0.get(&meld.discard_tile.unwrap()).unwrap();
                        let emoji = format_to_emoji(tile);
                        let current_player_hand =
                            game.game.table.hands.get(&current_player).unwrap();
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
