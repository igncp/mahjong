use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::{
    game::Players, table::BonusTiles, Board, Dragon, DragonTile, DrawWall, Flower, FlowerTile,
    Hand, HandTile, Hands, HandsMap, Season, SeasonTile, Suit, SuitTile, Table, Tile, TileId, Wind,
    WindTile,
};

pub type DeckContent = FxHashMap<TileId, Tile>;

fn create_deck_content() -> DeckContent {
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

    let mut deck: DeckContent = FxHashMap::default();

    deck_list.iter().enumerate().for_each(|(index, tile)| {
        let mut tile = tile.clone();
        let id = i32::try_from(index).unwrap();
        tile.set_id(id);
        deck.insert(id, tile);
    });

    deck
}

lazy_static! {
    static ref DECK_CONTENT: DeckContent = create_deck_content();
    pub static ref DEFAULT_DECK: Deck = Deck(DECK_CONTENT.clone());
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deck(pub DeckContent);

impl Deck {
    pub fn find_tile_without_id(tile: Tile) -> Tile {
        let all_tiles = DECK_CONTENT.values().cloned().collect::<Vec<Tile>>();

        all_tiles
            .iter()
            .find(|t| tile.is_same_content(t))
            .unwrap()
            .clone()
    }

    pub fn create_table(&self, players: &Players) -> Table {
        let Self(deck_content) = self;
        let mut draw_wall = DrawWall(deck_content.keys().cloned().collect::<Vec<TileId>>());

        draw_wall.0.shuffle(&mut thread_rng());

        let hands_map = players
            .iter()
            .map(|player| {
                let mut hand = Hand::new(vec![]);
                for _ in 0..(hand.style.clone().unwrap_or_default().tiles_after_claim() - 1) {
                    let tile_id = draw_wall.0.pop().unwrap();
                    let tile = HandTile {
                        id: tile_id,
                        concealed: true,
                        set_id: None,
                    };

                    hand.push(tile);
                }
                (player.clone(), hand)
            })
            .collect::<HandsMap>();

        let bonus_tiles = BonusTiles::default();

        Table {
            board: Board::default(),
            draw_wall,
            bonus_tiles,
            hands: Hands(hands_map),
        }
    }

    pub fn get_sure(&self, id: TileId) -> &Tile {
        self.0.get(&id).unwrap()
    }
}
