import type { Flower } from "bindings/Flower";
import type { Tile } from "bindings/Tile";

export const getAllUniqueTiles = (): Tile[] =>
  ([] as Tile[])
    .concat(
      Array.from({ length: 9 })
        .map((_, num) => {
          const value = num + 1;

          return (["Dots", "Bamboo", "Characters"] as const).map((suit) => ({
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
      (["Plum", "Orchid", "Chrysanthemum", "Bamboo"] as Flower[]).map(
        (value) => ({
          Flower: { id: 0, value },
        })
      )
    )
    .concat(
      (["Red", "Green", "White"] as const).map((value) => ({
        Dragon: { id: 0, value },
      }))
    )
    .concat(
      (["North", "East", "South", "West"] as const).map((value) => ({
        Wind: { id: 0, value },
      }))
    )
    .concat(
      Object.values(["Summer", "Winter", "Spring", "Autumn"] as const).map(
        (value) => ({
          Season: { id: 0, value },
        })
      )
    );
