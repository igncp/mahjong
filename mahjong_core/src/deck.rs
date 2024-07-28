use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    game::Players, table::BonusTiles, Board, Dragon, DragonTile, DrawWall, Flower, FlowerTile,
    Hand, Hands, HandsMap, Season, SeasonTile, Suit, SuitTile, Table, Tile, TileId, Wind, WindTile,
};

pub type DeckContent = Vec<Tile>;

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

    let mut deck: DeckContent = vec![];

    deck_list.iter().enumerate().for_each(|(index, tile)| {
        let mut tile = tile.clone();
        tile.set_id(index);
        deck.push(tile);
    });

    deck
}

lazy_static! {
    pub static ref DEFAULT_DECK: Deck = Deck(create_deck_content());
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Deck(pub DeckContent);

impl Deck {
    pub fn find_tile_without_id(tile: Tile) -> Tile {
        let all_tiles = DEFAULT_DECK.0.clone();

        all_tiles
            .iter()
            .find(|t| tile.is_same_content(t))
            .unwrap()
            .clone()
    }

    pub fn create_table(&self, players: &Players) -> Table {
        let Self(deck_content) = self;
        let mut ids: Vec<usize> = vec![];
        for id in 0..deck_content.len() {
            ids.push(id);
        }
        let draw_wall = DrawWall::new(ids);

        let hands_map = players
            .iter()
            .map(|player| {
                let hand = Hand::default();
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
        &self.0[id]
    }
}
