#[cfg(test)]
mod test {
    use crate::meld::{get_is_pung, SetCheckOpts};
    use crate::{Flower, FlowerTile, Suit, SuitTile, Tile, Wind, WindTile};

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
            let deck = tiles
                .iter()
                .map(|tile| (tile.get_id(), tile.clone()))
                .collect();
            let opts = SetCheckOpts { deck, sub_hand };
            let is_pung = get_is_pung(opts);
            assert_eq!(is_pung, *expected_is_pung, "index: {index}");
        }
    }
}
