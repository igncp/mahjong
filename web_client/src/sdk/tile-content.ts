import type { Tile } from "bindings/Tile";

const prefix = "https://upload.wikimedia.org/wikipedia/commons/";

// https://en.wikipedia.org/wiki/Mahjong_tiles#Contents
export const getTileInfo = (tile: Tile): [string, string] | null => {
  if ("Flower" in tile) {
    switch (tile.Flower.value) {
      case "Plum":
        return [`${prefix}8/8b/MJh5-.svg`, "Plum (Flower)"];
      case "Orchid":
        return [`${prefix}b/b3/MJh6-.svg`, "Orchid (Flower)"];
      case "Chrysanthemum":
        return [`${prefix}b/b6/MJh7-.svg`, "Chrysanthemum (Flower)"];
      case "Bamboo":
        return [`${prefix}9/9c/MJh8-.svg`, "Bamboo (Flower)"];
      default:
        tile.Flower.value satisfies never;
    }
  }

  if ("Wind" in tile) {
    switch (tile.Wind.value) {
      case "East":
        return [`${prefix}9/90/MJf1-.svg`, "East (Wind)"];
      case "South":
        return [`${prefix}b/bb/MJf2-.svg`, "South (Wind)"];
      case "West":
        return [`${prefix}5/54/MJf3-.svg`, "West (Wind)"];
      case "North":
        return [`${prefix}d/df/MJf4-.svg`, "North (Wind)"];
      default:
        tile.Wind.value satisfies never;
    }
  }

  if ("Dragon" in tile) {
    switch (tile.Dragon.value) {
      case "Red":
        return [`${prefix}2/20/MJd1-.svg`, "Red (Dragon)"];
      case "Green":
        return [`${prefix}8/8c/MJd2-.svg`, "Green (Dragon)"];
      case "White":
        return [`${prefix}5/52/MJd3-.svg`, "White (Dragon)"];
      default:
        tile.Dragon.value satisfies never;
    }
  }

  if ("Season" in tile) {
    switch (tile.Season.value) {
      case "Spring":
        return [`${prefix}1/14/MJh1-.svg`, "Spring (Season)"];
      case "Summer":
        return [`${prefix}e/e0/MJh2-.svg`, "Summer (Season)"];
      case "Autumn":
        return [`${prefix}2/25/MJh3-.svg`, "Autumn (Season)"];
      case "Winter":
        return [`${prefix}b/b7/MJh4-.svg`, "Winter (Season)"];
      default:
        tile.Season.value satisfies never;
    }
  }

  if ("Suit" in tile) {
    switch (tile.Suit.suit) {
      case "Dots": {
        switch (tile.Suit.value) {
          case 1:
            return [`${prefix}b/b3/MJt1-.svg`, "1 (Dots)"];
          case 2:
            return [`${prefix}a/a4/MJt2-.svg`, "2 (Dots)"];
          case 3:
            return [`${prefix}4/44/MJt3-.svg`, "3 (Dots)"];
          case 4:
            return [`${prefix}6/66/MJt4-.svg`, "4 (Dots)"];
          case 5:
            return [`${prefix}7/72/MJt5-.svg`, "5 (Dots)"];
          case 6:
            return [`${prefix}8/86/MJt6-.svg`, "6 (Dots)"];
          case 7:
            return [`${prefix}6/6c/MJt7-.svg`, "7 (Dots)"];
          case 8:
            return [`${prefix}6/66/MJt8-.svg`, "8 (Dots)"];
          case 9:
            return [`${prefix}f/f5/MJt9-.svg`, "9 (Dots)"];
          default:
            break;
        }

        break;
      }

      case "Bamboo": {
        switch (tile.Suit.value) {
          case 1:
            return [`${prefix}e/e8/MJs1-.svg`, "1 (Bamboo)"];
          case 2:
            return [`${prefix}9/97/MJs2-.svg`, "2 (Bamboo)"];
          case 3:
            return [`${prefix}1/1f/MJs3-.svg`, "3 (Bamboo)"];
          case 4:
            return [`${prefix}b/b1/MJs4-.svg`, "4 (Bamboo)"];
          case 5:
            return [`${prefix}6/61/MJs5-.svg`, "5 (Bamboo)"];
          case 6:
            return [`${prefix}6/63/MJs6-.svg`, "6 (Bamboo)"];
          case 7:
            return [`${prefix}8/8a/MJs7-.svg`, "7 (Bamboo)"];
          case 8:
            return [`${prefix}b/be/MJs8-.svg`, "8 (Bamboo)"];
          case 9:
            return [`${prefix}f/f3/MJs9-.svg`, "9 (Bamboo)"];
        }

        break;
      }

      case "Characters": {
        switch (tile.Suit.value) {
          case 1:
            return [`${prefix}3/32/MJw1-.svg`, "1 (Characters)"];
          case 2:
            return [`${prefix}7/70/MJw2-.svg`, "2 (Characters)"];
          case 3:
            return [`${prefix}d/d0/MJw3-.svg`, "3 (Characters)"];
          case 4:
            return [`${prefix}6/6b/MJw4-.svg`, "4 (Characters)"];
          case 5:
            return [`${prefix}4/4b/MJw5-.svg`, "5 (Characters)"];
          case 6:
            return [`${prefix}4/4c/MJw6-.svg`, "6 (Characters)"];
          case 7:
            return [`${prefix}c/c0/MJw7-.svg`, "7 (Characters)"];
          case 8:
            return [`${prefix}d/d3/MJw8-.svg`, "8 (Characters)"];
          case 9:
            return [`${prefix}a/a9/MJw9-.svg`, "9 (Characters)"];
        }

        break;
      }

      default:
        tile.Suit.suit satisfies never;
    }
  }

  return null;
};

export const getIsSameTile = (tileA: Tile, tileB: Tile) => {
  if ("Flower" in tileA) {
    return "Flower" in tileB && tileA.Flower.value === tileB.Flower.value;
  }

  if ("Wind" in tileA) {
    return "Wind" in tileB && tileA.Wind.value === tileB.Wind.value;
  }

  if ("Dragon" in tileA) {
    return "Dragon" in tileB && tileA.Dragon.value === tileB.Dragon.value;
  }

  if ("Season" in tileA) {
    return "Season" in tileB && tileA.Season.value === tileB.Season.value;
  }

  if ("Suit" in tileA) {
    return (
      "Suit" in tileB &&
      tileA.Suit.suit === tileB.Suit.suit &&
      tileA.Suit.value === tileB.Suit.value
    );
  }
};
