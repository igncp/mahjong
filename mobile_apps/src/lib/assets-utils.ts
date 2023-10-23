import {
  Dragon,
  Flower,
  FlowerTile,
  Season,
  Suit,
  Tile,
  Wind,
} from "mahjong_sdk/dist/core";

export const getTileImageName = (tile: Tile) => {
  if ((tile as FlowerTile).Flower) {
    switch ((tile as FlowerTile).Flower.value) {
      case Flower.Plum:
        return "flower-plum";
      case Flower.Orchid:
        return "flower-orchid";
      case Flower.Chrysanthemum:
        return "flower-chrysanthemum";
      case Flower.Bamboo:
        return "flower-bamboo";
    }
  }

  if ("Wind" in tile) {
    switch (tile.Wind.value) {
      case Wind.East:
        return "wind-east";
      case Wind.South:
        return "wind-south";
      case Wind.West:
        return "wind-west";
      case Wind.North:
        return "wind-north";
    }
  }

  if ("Dragon" in tile) {
    switch (tile.Dragon.value) {
      case Dragon.Red:
        return "dragon-red";
      case Dragon.Green:
        return "dragon-green";
      case Dragon.White:
        return "dragon-white";
    }
  }

  if ("Season" in tile) {
    switch (tile.Season.value) {
      case Season.Spring:
        return "season-spring";
      case Season.Summer:
        return "season-summer";
      case Season.Autumn:
        return "season-autumn";
      case Season.Winter:
        return "season-winter";
    }
  }

  if ("Suit" in tile) {
    switch (tile.Suit.suit) {
      case Suit.Dots: {
        switch (tile.Suit.value) {
          case 1:
            return "dots-1";
          case 2:
            return "dots-2";
          case 3:
            return "dots-3";
          case 4:
            return "dots-4";
          case 5:
            return "dots-5";
          case 6:
            return "dots-6";
          case 7:
            return "dots-7";
          case 8:
            return "dots-8";
          case 9:
            return "dots-9";
          default:
            break;
        }
        break;
      }
      case Suit.Bamboo: {
        switch (tile.Suit.value) {
          case 1:
            return "bamboo-1";
          case 2:
            return "bamboo-2";
          case 3:
            return "bamboo-3";
          case 4:
            return "bamboo-4";
          case 5:
            return "bamboo-5";
          case 6:
            return "bamboo-6";
          case 7:
            return "bamboo-7";
          case 8:
            return "bamboo-8";
          case 9:
            return "bamboo-9";
        }
        break;
      }
      case Suit.Characters: {
        switch (tile.Suit.value) {
          case 1:
            return "characters-1";
          case 2:
            return "characters-2";
          case 3:
            return "characters-3";
          case 4:
            return "characters-4";
          case 5:
            return "characters-5";
          case 6:
            return "characters-6";
          case 7:
            return "characters-7";
          case 8:
            return "characters-8";
          case 9:
            return "characters-9";
        }
        break;
      }
    }
  }

  throw new Error(`Tile not found: ${JSON.stringify(tile)}`);
};
