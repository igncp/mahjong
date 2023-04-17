import {
  Dragon,
  Flower,
  FlowerTile,
  Season,
  Suit,
  Tile,
  Wind,
} from "src/lib/mahjong-service";

const prefix = "https://upload.wikimedia.org/wikipedia/commons/";

// https://en.wikipedia.org/wiki/Mahjong_tiles#Contents
export const getTileImage = (tile: Tile): string | null => {
  if ((tile as FlowerTile).Flower) {
    switch ((tile as FlowerTile).Flower.value) {
      case Flower.Plum:
        return `${prefix}8/8b/MJh5-.svg`;
      case Flower.Orchid:
        return `${prefix}b/b3/MJh6-.svg`;
      case Flower.Chrysanthemum:
        return `${prefix}b/b6/MJh7-.svg`;
      case Flower.Bamboo:
        return `${prefix}9/9c/MJh8-.svg`;
    }
  }

  if ("Wind" in tile) {
    switch (tile.Wind.value) {
      case Wind.East:
        return `${prefix}9/90/MJf1-.svg`;
      case Wind.South:
        return `${prefix}b/bb/MJf2-.svg`;
      case Wind.West:
        return `${prefix}5/54/MJf3-.svg`;
      case Wind.North:
        return `${prefix}d/df/MJf4-.svg`;
    }
  }

  if ("Dragon" in tile) {
    switch (tile.Dragon.value) {
      case Dragon.Red:
        return `${prefix}2/20/MJd1-.svg`;
      case Dragon.Green:
        return `${prefix}8/8c/MJd2-.svg`;
      case Dragon.White:
        return `${prefix}5/52/MJd3-.svg`;
    }
  }

  if ("Season" in tile) {
    switch (tile.Season.value) {
      case Season.Spring:
        return `${prefix}1/14/MJh1-.svg`;
      case Season.Summer:
        return `${prefix}e/e0/MJh2-.svg`;
      case Season.Autumn:
        return `${prefix}2/25/MJh3-.svg`;
      case Season.Winter:
        return `${prefix}b/b7/MJh4-.svg`;
    }
  }

  if ("Suit" in tile) {
    switch (tile.Suit.suit) {
      case Suit.Dots: {
        switch (tile.Suit.value) {
          case 1:
            return `${prefix}b/b3/MJt1-.svg`;
          case 2:
            return `${prefix}a/a4/MJt2-.svg`;
          case 3:
            return `${prefix}4/44/MJt3-.svg`;
          case 4:
            return `${prefix}6/66/MJt4-.svg`;
          case 5:
            return `${prefix}7/72/MJt5-.svg`;
          case 6:
            return `${prefix}8/86/MJt6-.svg`;
          case 7:
            return `${prefix}6/6c/MJt7-.svg`;
          case 8:
            return `${prefix}6/66/MJt8-.svg`;
          case 9:
            return `${prefix}f/f5/MJt9-.svg`;
          default:
            break;
        }
        break;
      }
      case Suit.Bamboo: {
        switch (tile.Suit.value) {
          case 1:
            return `${prefix}e/e8/MJs1-.svg`;
          case 2:
            return `${prefix}9/97/MJs2-.svg`;
          case 3:
            return `${prefix}1/1f/MJs3-.svg`;
          case 4:
            return `${prefix}b/b1/MJs4-.svg`;
          case 5:
            return `${prefix}6/61/MJs5-.svg`;
          case 6:
            return `${prefix}6/63/MJs6-.svg`;
          case 7:
            return `${prefix}8/8a/MJs7-.svg`;
          case 8:
            return `${prefix}b/be/MJs8-.svg`;
          case 9:
            return `${prefix}f/f3/MJs9-.svg`;
        }
        break;
      }
      case Suit.Characters: {
        switch (tile.Suit.value) {
          case 1:
            return `${prefix}3/32/MJw1-.svg`;
          case 2:
            return `${prefix}7/70/MJw2-.svg`;
          case 3:
            return `${prefix}d/d0/MJw3-.svg`;
          case 4:
            return `${prefix}6/6b/MJw4-.svg`;
          case 5:
            return `${prefix}4/4b/MJw5-.svg`;
          case 6:
            return `${prefix}4/4c/MJw6-.svg`;
          case 7:
            return `${prefix}c/c0/MJw7-.svg`;
          case 8:
            return `${prefix}d/d3/MJw8-.svg`;
          case 9:
            return `${prefix}a/a9/MJw9-.svg`;
        }
        break;
      }
    }
  }

  return null;
};

const TileImg = ({ tile }: { tile: Tile }) => {
  const image = getTileImage(tile);

  if (!image) {
    return null;
  }

  return <img src={image} style={{ height: "50px", width: "50px" }} />;
};

export default TileImg;
