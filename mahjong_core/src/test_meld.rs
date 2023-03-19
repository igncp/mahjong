#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::deck::DeckContent;
    use crate::meld::{
        get_is_chow, get_is_kong, get_is_pung, get_possible_melds, GetPossibleMelds, Meld,
        PlayerDiff, SetCheckOpts,
    };
    use crate::tile::TileId;
    use crate::{Deck, Flower, FlowerTile, Hand, HandTile, Suit, SuitTile, Tile, Wind, WindTile};

    type GetIsPungFixture = (Vec<Tile>, bool);
    fn get_is_pung_fixtures() -> Vec<GetIsPungFixture> {
        vec![
            (
                vec![
                    Tile::Suit(SuitTile {
                        id: 0,
                        value: 1,
                        suit: Suit::Dots,
                    }),
                    Tile::Suit(SuitTile {
                        id: 1,
                        value: 1,
                        suit: Suit::Dots,
                    }),
                    Tile::Suit(SuitTile {
                        id: 2,
                        value: 1,
                        suit: Suit::Dots,
                    }),
                ],
                true,
            ),
            (
                vec![
                    Tile::Suit(SuitTile {
                        id: 0,
                        value: 1,
                        suit: Suit::Dots,
                    }),
                    Tile::Suit(SuitTile {
                        id: 1,
                        value: 2,
                        suit: Suit::Dots,
                    }),
                    Tile::Suit(SuitTile {
                        id: 2,
                        value: 1,
                        suit: Suit::Dots,
                    }),
                ],
                // Not all the same value
                false,
            ),
            (
                vec![
                    Tile::Wind(WindTile {
                        id: 0,
                        value: Wind::North,
                    }),
                    Tile::Wind(WindTile {
                        id: 1,
                        value: Wind::North,
                    }),
                    Tile::Wind(WindTile {
                        id: 2,
                        value: Wind::North,
                    }),
                ],
                true,
            ),
            // Bonus are never valid pungs
            (
                vec![
                    Tile::Flower(FlowerTile {
                        id: 0,
                        value: Flower::Bamboo,
                    }),
                    Tile::Flower(FlowerTile {
                        id: 1,
                        value: Flower::Bamboo,
                    }),
                    Tile::Flower(FlowerTile {
                        id: 2,
                        value: Flower::Bamboo,
                    }),
                ],
                // Not all the same value
                false,
            ),
            (
                vec![
                    Tile::Wind(WindTile {
                        id: 0,
                        value: Wind::West,
                    }),
                    Tile::Wind(WindTile {
                        id: 1,
                        value: Wind::West,
                    }),
                    Tile::Suit(SuitTile {
                        id: 2,
                        value: 7,
                        suit: Suit::Dots,
                    }),
                ],
                false,
            ),
        ]
    }

    #[test]
    fn test_is_pung() {
        for (index, (tiles, expected_is_pung)) in get_is_pung_fixtures().iter().enumerate() {
            let sub_hand = tiles.iter().map(|tile| tile.get_id()).collect();
            let deck_content: DeckContent = tiles
                .iter()
                .map(|tile| (tile.get_id(), tile.clone()))
                .collect();
            let deck = Deck(deck_content);
            let opts = SetCheckOpts {
                board_tile_player_diff: None,
                claimed_tile: None,
                deck: &deck,
                sub_hand: &sub_hand,
            };
            let is_pung = get_is_pung(&opts);
            assert_eq!(is_pung, *expected_is_pung, "index: {index}");
        }
    }

    type GetIsChowFixture = (Vec<Tile>, bool, Option<TileId>, PlayerDiff);
    fn get_is_chow_fixtures() -> Vec<GetIsChowFixture> {
        vec![
            (
                vec![
                    Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 0,
                    }),
                    Tile::Suit(SuitTile {
                        value: 3,
                        suit: Suit::Dots,
                        id: 1,
                    }),
                    Tile::Suit(SuitTile {
                        value: 2,
                        suit: Suit::Dots,
                        id: 2,
                    }),
                ],
                true,
                None,
                None,
            ),
            (
                vec![
                    Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 0,
                    }),
                    Tile::Suit(SuitTile {
                        value: 3,
                        suit: Suit::Dots,
                        id: 1,
                    }),
                    Tile::Suit(SuitTile {
                        value: 2,
                        suit: Suit::Dots,
                        id: 2,
                    }),
                ],
                true,
                Some(1),
                Some(1),
            ),
            (
                vec![
                    Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 0,
                    }),
                    Tile::Suit(SuitTile {
                        value: 3,
                        suit: Suit::Dots,
                        id: 1,
                    }),
                    Tile::Suit(SuitTile {
                        value: 2,
                        suit: Suit::Dots,
                        id: 2,
                    }),
                ],
                false,
                Some(1),
                Some(-1),
            ),
            (
                vec![
                    Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 0,
                    }),
                    Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 1,
                    }),
                    Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 2,
                    }),
                ],
                false,
                Some(1),
                None,
            ),
        ]
    }

    #[test]
    fn test_is_chow() {
        for (index, (tiles, expected_is_chow, claimed_tile, board_tile_player_diff)) in
            get_is_chow_fixtures().iter().enumerate()
        {
            let sub_hand = tiles.iter().map(|tile| tile.get_id()).collect();
            let deck = Deck(
                tiles
                    .iter()
                    .map(|tile| (tile.get_id(), tile.clone()))
                    .collect(),
            );
            let opts = SetCheckOpts {
                board_tile_player_diff: *board_tile_player_diff,
                claimed_tile: *claimed_tile,
                deck: &deck,
                sub_hand: &sub_hand,
            };
            let is_chow = get_is_chow(&opts);
            assert_eq!(is_chow, *expected_is_chow, "index: {index}");
        }
    }

    type GetIsKongFixture = (Vec<Tile>, bool);
    fn get_is_kong_fixtures() -> Vec<GetIsKongFixture> {
        vec![(
            vec![
                Tile::Wind(WindTile {
                    id: 0,
                    value: Wind::West,
                }),
                Tile::Wind(WindTile {
                    id: 1,
                    value: Wind::West,
                }),
                Tile::Suit(SuitTile {
                    id: 2,
                    value: 7,
                    suit: Suit::Dots,
                }),
                Tile::Suit(SuitTile {
                    id: 3,
                    value: 7,
                    suit: Suit::Dots,
                }),
            ],
            false,
        )]
    }

    #[test]
    fn test_get_is_kong() {
        for (index, (tiles, expected_is_kong)) in get_is_kong_fixtures().iter().enumerate() {
            let sub_hand = tiles.iter().map(|tile| tile.get_id()).collect();
            let deck = Deck(
                tiles
                    .iter()
                    .map(|tile| (tile.get_id(), tile.clone()))
                    .collect(),
            );
            let opts = SetCheckOpts {
                board_tile_player_diff: None,
                claimed_tile: None,
                deck: &deck,
                sub_hand: &sub_hand,
            };
            let is_chow = get_is_kong(&opts);
            assert_eq!(is_chow, *expected_is_kong, "index: {index}");
        }
    }

    type GetPossibleMeldsFixture = (Hand, Deck, PlayerDiff, Vec<Meld>);
    fn get_possible_melds_fixtures() -> Vec<GetPossibleMeldsFixture> {
        fn get_hand_tile(id: TileId) -> HandTile {
            HandTile {
                id,
                concealed: true,
                set_id: None,
            }
        }
        fn get_tile(id: TileId) -> (TileId, Tile) {
            (
                id,
                Tile::Suit(SuitTile {
                    value: 2,
                    suit: Suit::Dots,
                    id,
                }),
            )
        }
        vec![(
            Hand(vec![
                get_hand_tile(0),
                get_hand_tile(1),
                get_hand_tile(2),
                get_hand_tile(3),
            ]),
            Deck(HashMap::from_iter(vec![
                get_tile(0),
                get_tile(1),
                get_tile(2),
                get_tile(3),
            ])),
            Some(0),
            vec![
                vec![0, 1, 2],
                vec![0, 1, 2, 3],
                vec![0, 1, 3],
                vec![0, 2, 3],
                vec![1, 2, 3],
            ],
        )]
    }

    #[test]
    fn test_get_possible_melds() {
        for (index, (hand, deck, player_diff, expected_meld)) in
            get_possible_melds_fixtures().iter().enumerate()
        {
            let opts = GetPossibleMelds {
                hand,
                deck,
                board_tile_player_diff: *player_diff,
                claimed_tile: None,
            };
            let possible_melds = get_possible_melds(&opts);
            assert_eq!(possible_melds, *expected_meld, "index: {index}");
        }
    }
}
