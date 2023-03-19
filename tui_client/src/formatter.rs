use mahjong_core::{Dragon, Flower, Game, Season, Suit, Tile, Wind};

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

pub fn get_hand_str(game: &Game) -> Vec<String> {
    let mut lines = vec![];
    let current_player = game.get_current_player();

    for player in game.players.iter() {
        let hand = game.table.hands.get(&player.id).unwrap();

        lines.push("".to_string());
        lines.push(format!(
            "{}{}:",
            player.name,
            if player.id == current_player.id {
                " *"
            } else {
                ""
            }
        ));

        let line = hand
            .0
            .iter()
            .enumerate()
            .map(|(idx, hand_tile)| {
                let tile = game.deck.0.get(&hand_tile.id).unwrap();
                let tile_str = format_to_emoji(tile);
                let idx_formatted = format!("{:0>2}", idx);
                format!("[{idx_formatted}]({tile_str})")
            })
            .collect::<Vec<String>>()
            .join("  ");

        lines.push(line.clone());
    }

    lines

    // let [, playerIndex] = input.split(" ").map(Number);
    // const { hands } = game.table;
    // const { deck, players, round } = game;
    // if (playerIndex >= players.length) return;
    // const printedPlayers =
    //   Number.isNaN(playerIndex) || typeof playerIndex !== "number"
    //     ? players
    //     : [players[playerIndex]];
    // printedPlayers.forEach((player) => {
    //   const playerId = player.id;
    //   const hand = hands[playerId];
    //   const { melds } = getHandMelds({ hand });
    //   const formatter = pretty
    //     ? (handTile: HandTile, tileIndex: number) => {
    //         const { id } = handTile;
    //         return `[${tileIndex.toString().padStart(2, "0")}](${formatToEmoji(
    //           deck[id]
    //         )})`;
    //       }
    //     : (handTile: HandTile, tileIndex: number) => {
    //         const { id, ...rest } = handTile;
    //         return `- [${tileIndex.toString().padStart(2, "0")}] ${JSON.stringify(
    //           rest
    //         )} | ${JSON.stringify(deck[id])}`;
    //       };
    //   const joiner = pretty ? " " : "\n";
    //   console.log("");
    //   console.log(
    //     "Player: " +
    //       player.name +
    //       (player.id === players[round.playerIndex].id ? "*" : "")
    //   );
    //   console.log("Total: " + hand.length);
    //   console.log(
    //     [
    //       hand
    //         .filter((h) => !h.setId)
    //         .map(formatter)
    //         .join(joiner),
    //       ...Object.keys(melds).map((setId) => {
    //         const meld = melds[setId as keyof typeof melds];
    //         if (!meld) return "";
    //         return (
    //           "- Meld: " +
    //           meld.map(formatter).join(joiner) +
    //           " (" +
    //           setId +
    //           ") (concealed: " +
    //           meld.every((h) => h.concealed) +
    //           ")"
    //         );
    //       }),
    //     ].join("\n")
    //   );
    //   console.log("");
    // });
}
