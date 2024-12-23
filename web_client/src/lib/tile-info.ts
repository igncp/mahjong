import type { Tile } from "bindings/Tile";
import type { getI18n } from "react-i18next";

import type { TileId } from "src/sdk/core";

const prefix = "https://upload.wikimedia.org/wikipedia/commons/";

// https://en.wikipedia.org/wiki/Mahjong_tiles#Contents
export const getTileInfo = (
  tile: Tile,
  i18n: ReturnType<typeof getI18n>,
): [string, string] | null => {
  if ("Flower" in tile) {
    switch (tile.Flower.value) {
      case "Plum":
        return [`${prefix}8/8b/MJh5-.svg`, i18n.t("tile.plum")];
      case "Orchid":
        return [`${prefix}b/b3/MJh6-.svg`, i18n.t("tile.orchid")];
      case "Chrysanthemum":
        return [`${prefix}b/b6/MJh7-.svg`, i18n.t("tile.chrys")];
      case "Bamboo":
        return [`${prefix}9/9c/MJh8-.svg`, i18n.t("tile.bamboo")];
      default:
        tile.Flower.value satisfies never;
    }
  }

  if ("Wind" in tile) {
    switch (tile.Wind.value) {
      case "East":
        return [`${prefix}9/90/MJf1-.svg`, i18n.t("tile.east")];
      case "South":
        return [`${prefix}b/bb/MJf2-.svg`, i18n.t("tile.south")];
      case "West":
        return [`${prefix}5/54/MJf3-.svg`, i18n.t("tile.west")];
      case "North":
        return [`${prefix}d/df/MJf4-.svg`, i18n.t("tile.north")];
      default:
        tile.Wind.value satisfies never;
    }
  }

  if ("Dragon" in tile) {
    switch (tile.Dragon.value) {
      case "Red":
        return [`${prefix}2/20/MJd1-.svg`, i18n.t("tile.red")];
      case "Green":
        return [`${prefix}8/8c/MJd2-.svg`, i18n.t("tile.green")];
      case "White":
        return [`${prefix}5/52/MJd3-.svg`, i18n.t("tile.white")];
      default:
        tile.Dragon.value satisfies never;
    }
  }

  if ("Season" in tile) {
    switch (tile.Season.value) {
      case "Spring":
        return [`${prefix}1/14/MJh1-.svg`, i18n.t("tile.spring")];
      case "Summer":
        return [`${prefix}e/e0/MJh2-.svg`, i18n.t("tile.summer")];
      case "Autumn":
        return [`${prefix}2/25/MJh3-.svg`, i18n.t("tile.autumn")];
      case "Winter":
        return [`${prefix}b/b7/MJh4-.svg`, i18n.t("tile.winter")];
      default:
        tile.Season.value satisfies never;
    }
  }

  if ("Suit" in tile) {
    switch (tile.Suit.suit) {
      case "Dots": {
        switch (tile.Suit.value) {
          case 1:
            return [`${prefix}b/b3/MJt1-.svg`, i18n.t("tile.dots1")];
          case 2:
            return [`${prefix}a/a4/MJt2-.svg`, i18n.t("tile.dots2")];
          case 3:
            return [`${prefix}4/44/MJt3-.svg`, i18n.t("tile.dots3")];
          case 4:
            return [`${prefix}6/66/MJt4-.svg`, i18n.t("tile.dots4")];
          case 5:
            return [`${prefix}7/72/MJt5-.svg`, i18n.t("tile.dots5")];
          case 6:
            return [`${prefix}8/86/MJt6-.svg`, i18n.t("tile.dots6")];
          case 7:
            return [`${prefix}6/6c/MJt7-.svg`, i18n.t("tile.dots7")];
          case 8:
            return [`${prefix}6/66/MJt8-.svg`, i18n.t("tile.dots8")];
          case 9:
            return [`${prefix}f/f5/MJt9-.svg`, i18n.t("tile.dots9")];
          default:
            break;
        }

        break;
      }

      case "Bamboo": {
        switch (tile.Suit.value) {
          case 1:
            return [`${prefix}e/e8/MJs1-.svg`, i18n.t("tile.bamboo1")];
          case 2:
            return [`${prefix}9/97/MJs2-.svg`, i18n.t("tile.bamboo2")];
          case 3:
            return [`${prefix}1/1f/MJs3-.svg`, i18n.t("tile.bamboo3")];
          case 4:
            return [`${prefix}b/b1/MJs4-.svg`, i18n.t("tile.bamboo4")];
          case 5:
            return [`${prefix}6/61/MJs5-.svg`, i18n.t("tile.bamboo5")];
          case 6:
            return [`${prefix}6/63/MJs6-.svg`, i18n.t("tile.bamboo6")];
          case 7:
            return [`${prefix}8/8a/MJs7-.svg`, i18n.t("tile.bamboo7")];
          case 8:
            return [`${prefix}b/be/MJs8-.svg`, i18n.t("tile.bamboo8")];
          case 9:
            return [`${prefix}f/f3/MJs9-.svg`, i18n.t("tile.bamboo9")];
        }

        break;
      }

      case "Characters": {
        switch (tile.Suit.value) {
          case 1:
            return [`${prefix}3/32/MJw1-.svg`, i18n.t("tile.chars1")];
          case 2:
            return [`${prefix}7/70/MJw2-.svg`, i18n.t("tile.chars2")];
          case 3:
            return [`${prefix}d/d0/MJw3-.svg`, i18n.t("tile.chars3")];
          case 4:
            return [`${prefix}6/6b/MJw4-.svg`, i18n.t("tile.chars4")];
          case 5:
            return [`${prefix}4/4b/MJw5-.svg`, i18n.t("tile.chars5")];
          case 6:
            return [`${prefix}4/4c/MJw6-.svg`, i18n.t("tile.chars6")];
          case 7:
            return [`${prefix}c/c0/MJw7-.svg`, i18n.t("tile.chars7")];
          case 8:
            return [`${prefix}d/d3/MJw8-.svg`, i18n.t("tile.chars8")];
          case 9:
            return [`${prefix}a/a9/MJw9-.svg`, i18n.t("tile.chars9")];
        }

        break;
      }

      default:
        tile.Suit.suit satisfies never;
    }
  }

  return null;
};

export const getTileId = (tile: Tile): null | TileId => {
  if ("Flower" in tile) {
    return tile.Flower.id;
  }

  if ("Wind" in tile) {
    return tile.Wind.id;
  }

  if ("Dragon" in tile) {
    return tile.Dragon.id;
  }

  if ("Season" in tile) {
    return tile.Season.id;
  }

  if ("Suit" in tile) {
    return tile.Suit.id;
  }

  return null;
};
