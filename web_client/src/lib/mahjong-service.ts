// These are maintained manually. An alternative would be to use:
// https://github.com/Aleph-Alpha/ts-rs or OpenAPI spec but for now it is
// faster to maintain manually.

export type GameId = string;
export type PlayerId = string;
export type TileId = number;

export enum Flower {
  Bamboo = "Bamboo",
  Chrysanthemum = "Chrysanthemum",
  Orchid = "Orchid",
  Plum = "Plum",
}

type FlowerTile = {
  Flower: {
    id: TileId;
    value: Flower;
  };
};

type Tile = FlowerTile;

export type Game = {
  deck: Record<TileId, Tile>;
  id: GameId;
  name: string;
  table: {
    draw_wall: TileId[];
  };
};

export type ServicePlayer = {
  id: string;
  name: string;
};

export type ServiceGame = {
  game: Game;
  players: Record<PlayerId, ServicePlayer>;
};

export type TAdminGetGamesResponse = GameId[];
export type TAdminGetGameResponse = ServiceGame;

export type TAdminPostNewGameResponse = ServiceGame;
