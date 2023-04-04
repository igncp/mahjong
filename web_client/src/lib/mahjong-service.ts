import { format_tile, get_possible_melds } from "pkg";

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

export type Tile = FlowerTile;
export type SetId = string;

type HandTile = {
  concealed: boolean;
  id: TileId;
  set_id: SetId;
};
type Hand = HandTile[];
type Hands = Record<PlayerId, Hand>;
type Score = Record<PlayerId, number>;

export type Game = {
  deck: Record<TileId, Tile>;
  id: GameId;
  name: string;
  players: PlayerId[];
  round: {
    player_index: PlayerId;
  };
  score: Score;
  table: {
    board: TileId[];
    draw_wall: TileId[];
    hands: Hands;
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

export type GameSummary = {
  id: GameId;
  score: Score;
};

export type ServiceGameSummary = {
  game_summary: GameSummary;
};

export type TAdminPostBreakMeldRequest = {
  player_id: PlayerId;
  set_id: string;
};
export type TAdminPostBreakMeldResponse = Hand;

export type TAdminPostAIContinueRequest = {
  draw?: boolean;
};
export type TAdminPostAIContinueResponse = {
  changed: boolean;
  service_game: ServiceGame;
};

export type TAdminPostCreateMeldRequest = {
  player_id: PlayerId;
  tiles: TileId[];
};
export type TAdminPostCreateMeldResponse = Hand;

export type TAdminGetGamesResponse = GameId[];
export type TAdminGetGameResponse = ServiceGame;

export type TAdminPostNewGameRequest = void;
export type TAdminPostNewGameResponse = ServiceGame;

export type TAdminPostDrawCardRequest = void;
export type TAdminPostDrawCardResponse = Hand;

export type TAdminPostDrawWallSwapTilesRequest = {
  tile_id_a: TileId;
  tile_id_b: TileId;
};
export type TAdminPostDrawWallSwapTilesResponse = ServiceGame;

export type TAdminPostDiscardTileRequest = {
  tile_id: TileId;
};
export type TAdminPostDiscardTileResponse = ServiceGame;

export type TAdminPostMovePlayerRequest = void;
export type TAdminPostMovePlayerResponse = ServiceGame;

export type TAdminPostSortHandsRequest = void;
export type TAdminPostSortHandsResponse = Hands;

export type TAdminPostSayMahjongRequest = {
  player_id: PlayerId;
};
export type TAdminPostSayMahjongResponse = ServiceGame;

export type PossibleMeld = {
  discard_tile: unknown;
  player_id: PlayerId;
  tiles: TileId[];
};

export type TSocketMessage = {
  GameUpdate: ServiceGame;
};

export type TUserGetGamesQuery = {
  player_id: PlayerId;
};
export type TUserGetGamesResponse = GameId[];

export type TUserLoadGameQuery = {
  player_id: PlayerId;
};
export type TUserLoadGameResponse = ServiceGameSummary;

export class ModelServiceGame {
  constructor(
    public data: ServiceGame,
    public setGame: (g: ServiceGame) => void
  ) {}

  getCurrentPlayer() {
    const playerId = this.data.game.players[this.data.game.round.player_index];

    return this.data.players[playerId];
  }

  getTileString(tileId: TileId) {
    const tile = this.data.game.deck[tileId];
    const tileString = format_tile(tile);

    return `[${tileString}]`;
  }

  getPlayerScore(playerId: PlayerId) {
    return this.data.game.score[playerId];
  }

  getPossibleMelds(): PossibleMeld[] {
    const possibleMelds = get_possible_melds(JSON.stringify(this.data));

    return possibleMelds;
  }
}
