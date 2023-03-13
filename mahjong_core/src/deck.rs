use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

use crate::{
    Deck, Dragon, DragonTile, Flower, FlowerTile, HandTile, Hands, Player, Season, SeasonTile,
    Suit, SuitTile, Table, Tile, TileId, Wind, WindTile,
};

pub fn get_default_deck() -> Deck {
    let suits_set = vec![Suit::Bamboo, Suit::Dots, Suit::Characters];
    let winds_set = vec![Wind::East, Wind::North, Wind::South, Wind::West];
    let dragons_set = vec![Dragon::Green, Dragon::Red, Dragon::White];
    let seasons_set = vec![
        Season::Spring,
        Season::Summer,
        Season::Autumn,
        Season::Winter,
    ];
    let flowers_set = vec![
        Flower::Bamboo,
        Flower::Chrysanthemum,
        Flower::Orchid,
        Flower::Plum,
    ];

    let mut deck_list: Vec<Tile> = vec![];

    for flower in flowers_set {
        deck_list.push(Tile::Flower(FlowerTile {
            id: 0,
            value: flower,
        }));
    }

    for season in seasons_set {
        deck_list.push(Tile::Season(SeasonTile {
            id: 0,
            value: season,
        }));
    }

    for _ in 0..4 {
        for suit in suits_set.clone() {
            for value in 1..10 {
                let suit_tile = SuitTile { id: 0, value, suit };
                let tile = Tile::Suit(suit_tile);

                deck_list.push(tile);
            }
        }

        for wind in winds_set.clone() {
            let wind_tile = WindTile { id: 0, value: wind };
            let tile = Tile::Wind(wind_tile);

            deck_list.push(tile);
        }

        for dragon in dragons_set.clone() {
            let dragon_tile = DragonTile {
                id: 0,
                value: dragon,
            };
            let tile = Tile::Dragon(dragon_tile);

            deck_list.push(tile);
        }
    }

    let mut deck: Deck = HashMap::new();

    deck_list.iter().enumerate().for_each(|(index, tile)| {
        let mut tile = tile.clone();
        let id = u32::try_from(index).unwrap();
        tile.set_id(id);
        deck.insert(id, tile);
    });

    deck
}

pub fn create_table(deck: &Deck, players: &[Player]) -> Table {
    let mut draw_wall = deck.keys().cloned().collect::<Vec<TileId>>();

    draw_wall.shuffle(&mut thread_rng());

    let hands = players
        .iter()
        .map(|player| {
            let mut hand: Vec<HandTile> = vec![];
            for _ in 0..13 {
                let tile_id = draw_wall.pop().unwrap();
                let tile = HandTile {
                    id: tile_id,
                    concealed: true,
                    set_id: None,
                };

                hand.push(tile);
            }
            (player.id.clone(), hand)
        })
        .collect::<Hands>();

    Table {
        board: vec![],
        draw_wall,
        hands,
    }
}
