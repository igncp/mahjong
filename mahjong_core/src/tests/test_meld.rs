#[cfg(test)]
mod test {
    use crate::hand::HandPossibleMeld;
    use crate::meld::{get_is_chow, get_is_kong, get_is_pung, PlayerDiff, SetCheckOpts};
    use crate::tile::TileId;
    use crate::{Deck, Flower, FlowerTile, Hand, HandTile, Suit, SuitTile, Tile, Wind, WindTile};
    use pretty_assertions::assert_eq;

    type GetIsPungFixture = (Vec<Tile>, bool);
    fn get_is_pung_fixtures() -> Vec<GetIsPungFixture> {
        vec![
            (
                vec![
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        id: 0,
                        value: 1,
                        suit: Suit::Dots,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        id: 0,
                        value: 1,
                        suit: Suit::Dots,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        id: 0,
                        value: 1,
                        suit: Suit::Dots,
                    })),
                ],
                true,
            ),
            (
                vec![
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        id: 0,
                        value: 1,
                        suit: Suit::Dots,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        id: 1,
                        value: 2,
                        suit: Suit::Dots,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        id: 2,
                        value: 1,
                        suit: Suit::Dots,
                    })),
                ],
                // Not all the same value
                false,
            ),
            (
                vec![
                    Deck::find_tile_without_id(Tile::Wind(WindTile {
                        id: 0,
                        value: Wind::North,
                    })),
                    Deck::find_tile_without_id(Tile::Wind(WindTile {
                        id: 1,
                        value: Wind::North,
                    })),
                    Deck::find_tile_without_id(Tile::Wind(WindTile {
                        id: 2,
                        value: Wind::North,
                    })),
                ],
                true,
            ),
            // Bonus are never valid pungs
            (
                vec![
                    Deck::find_tile_without_id(Tile::Flower(FlowerTile {
                        id: 0,
                        value: Flower::Bamboo,
                    })),
                    Deck::find_tile_without_id(Tile::Flower(FlowerTile {
                        id: 1,
                        value: Flower::Bamboo,
                    })),
                    Deck::find_tile_without_id(Tile::Flower(FlowerTile {
                        id: 2,
                        value: Flower::Bamboo,
                    })),
                ],
                // Not all the same value
                false,
            ),
            (
                vec![
                    Deck::find_tile_without_id(Tile::Wind(WindTile {
                        id: 0,
                        value: Wind::West,
                    })),
                    Deck::find_tile_without_id(Tile::Wind(WindTile {
                        id: 0,
                        value: Wind::West,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        id: 0,
                        value: 7,
                        suit: Suit::Dots,
                    })),
                ],
                false,
            ),
        ]
    }

    #[test]
    fn test_is_pung() {
        for (index, (tiles, expected_is_pung)) in get_is_pung_fixtures().iter().enumerate() {
            let sub_hand = tiles.iter().map(|tile| tile.get_id()).collect();
            let opts = SetCheckOpts {
                board_tile_player_diff: None,
                claimed_tile: None,
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
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 0,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 3,
                        suit: Suit::Dots,
                        id: 1,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 2,
                        suit: Suit::Dots,
                        id: 2,
                    })),
                ],
                true,
                None,
                None,
            ),
            (
                vec![
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 0,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 3,
                        suit: Suit::Dots,
                        id: 1,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 2,
                        suit: Suit::Dots,
                        id: 2,
                    })),
                ],
                true,
                Some(
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 3,
                        suit: Suit::Dots,
                        id: 1,
                    }))
                    .get_id(),
                ),
                Some(1),
            ),
            (
                vec![
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 0,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 3,
                        suit: Suit::Dots,
                        id: 1,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 2,
                        suit: Suit::Dots,
                        id: 2,
                    })),
                ],
                false,
                Some(
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 3,
                        suit: Suit::Dots,
                        id: 1,
                    }))
                    .get_id(),
                ),
                Some(-1),
            ),
            (
                vec![
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 0,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 1,
                    })),
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 1,
                        suit: Suit::Dots,
                        id: 2,
                    })),
                ],
                false,
                Some(
                    Deck::find_tile_without_id(Tile::Suit(SuitTile {
                        value: 3,
                        suit: Suit::Dots,
                        id: 1,
                    }))
                    .get_id(),
                ),
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
            let opts = SetCheckOpts {
                board_tile_player_diff: *board_tile_player_diff,
                claimed_tile: *claimed_tile,
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
            let opts = SetCheckOpts {
                board_tile_player_diff: None,
                claimed_tile: None,
                sub_hand: &sub_hand,
            };
            let is_chow = get_is_kong(&opts);
            assert_eq!(is_chow, *expected_is_kong, "index: {index}");
        }
    }

    type GetPossibleMeldsFixture = (Hand, PlayerDiff, Vec<HandPossibleMeld>);
    fn get_possible_melds_fixtures() -> Vec<GetPossibleMeldsFixture> {
        fn get_hand_tile(tile: &Tile) -> HandTile {
            HandTile {
                id: tile.get_id(),
                concealed: true,
                set_id: None,
            }
        }

        let first_tile = Deck::find_tile_without_id(Tile::Suit(SuitTile {
            value: 1,
            suit: Suit::Dots,
            id: 0,
        }));
        let second_tile = Deck::find_tile_without_id(Tile::Suit(SuitTile {
            value: 2,
            suit: Suit::Dots,
            id: 0,
        }));
        let third_tile = Deck::find_tile_without_id(Tile::Suit(SuitTile {
            value: 3,
            suit: Suit::Dots,
            id: 0,
        }));

        vec![(
            Hand(vec![
                get_hand_tile(&first_tile),
                get_hand_tile(&second_tile),
                get_hand_tile(&third_tile),
            ]),
            Some(0),
            vec![HandPossibleMeld {
                is_mahjong: false,
                tiles: vec![
                    first_tile.get_id(),
                    second_tile.get_id(),
                    third_tile.get_id(),
                ],
            }],
        )]
    }

    #[test]
    fn test_get_possible_melds() {
        for (index, (hand, player_diff, expected_meld)) in
            get_possible_melds_fixtures().iter().enumerate()
        {
            let possible_melds = hand.get_possible_melds(*player_diff, None, false);
            assert_eq!(possible_melds, *expected_meld, "index: {index}");
        }
    }
}
