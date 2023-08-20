import { Dragon, Flower, Season, Suit, Tile, Wind } from "./core";

export const getAllUniqueTiles = (): Tile[] =>
  ([] as Tile[])
    .concat(
      Array.from({ length: 9 })
        .map((_, num) => {
          const value = num + 1;
          return [Suit.Dots, Suit.Bamboo, Suit.Characters].map((suit) => ({
            Suit: {
              id: 0,
              suit,
              value,
            },
          }));
        })
        .flat()
    )
    .concat(
      Object.values(Flower).map((value) => ({
        Flower: { id: 0, value },
      }))
    )
    .concat(
      Object.values(Dragon).map((value) => ({
        Dragon: { id: 0, value },
      }))
    )
    .concat(
      Object.values(Wind).map((value) => ({
        Wind: { id: 0, value },
      }))
    )
    .concat(
      Object.values(Season).map((value) => ({
        Season: { id: 0, value },
      }))
    );
