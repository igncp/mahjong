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

export const imageNameToImport = {
  "bamboo-1": require("../../assets/pngs/bamboo-1.png"),
  "bamboo-2": require("../../assets/pngs/bamboo-2.png"),
  "bamboo-3": require("../../assets/pngs/bamboo-3.png"),
  "bamboo-4": require("../../assets/pngs/bamboo-4.png"),
  "bamboo-5": require("../../assets/pngs/bamboo-5.png"),
  "bamboo-6": require("../../assets/pngs/bamboo-6.png"),
  "bamboo-7": require("../../assets/pngs/bamboo-7.png"),
  "bamboo-8": require("../../assets/pngs/bamboo-8.png"),
  "bamboo-9": require("../../assets/pngs/bamboo-9.png"),
  "characters-1": require("../../assets/pngs/characters-1.png"),
  "characters-2": require("../../assets/pngs/characters-2.png"),
  "characters-3": require("../../assets/pngs/characters-3.png"),
  "characters-4": require("../../assets/pngs/characters-4.png"),
  "characters-5": require("../../assets/pngs/characters-5.png"),
  "characters-6": require("../../assets/pngs/characters-6.png"),
  "characters-7": require("../../assets/pngs/characters-7.png"),
  "characters-8": require("../../assets/pngs/characters-8.png"),
  "characters-9": require("../../assets/pngs/characters-9.png"),
  "dots-1": require("../../assets/pngs/dots-1.png"),
  "dots-2": require("../../assets/pngs/dots-2.png"),
  "dots-3": require("../../assets/pngs/dots-3.png"),
  "dots-4": require("../../assets/pngs/dots-4.png"),
  "dots-5": require("../../assets/pngs/dots-5.png"),
  "dots-6": require("../../assets/pngs/dots-6.png"),
  "dots-7": require("../../assets/pngs/dots-7.png"),
  "dots-8": require("../../assets/pngs/dots-8.png"),
  "dots-9": require("../../assets/pngs/dots-9.png"),
  "dragon-green": require("../../assets/pngs/dragon-green.png"),
  "dragon-red": require("../../assets/pngs/dragon-red.png"),
  "dragon-white": require("../../assets/pngs/dragon-white.png"),
  "flower-bamboo": require("../../assets/pngs/flower-bamboo.png"),
  "flower-chrysanthemum": require("../../assets/pngs/flower-chrysanthemum.png"),
  "flower-orchid": require("../../assets/pngs/flower-orchid.png"),
  "flower-plum": require("../../assets/pngs/flower-plum.png"),
  "season-autumn": require("../../assets/pngs/season-autumn.png"),
  "season-spring": require("../../assets/pngs/season-spring.png"),
  "season-summer": require("../../assets/pngs/season-summer.png"),
  "season-winter": require("../../assets/pngs/season-winter.png"),
  "wind-east": require("../../assets/pngs/wind-east.png"),
  "wind-north": require("../../assets/pngs/wind-north.png"),
  "wind-south": require("../../assets/pngs/wind-south.png"),
  "wind-west": require("../../assets/pngs/wind-west.png"),
};
