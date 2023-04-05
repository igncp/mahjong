import {
  format_tile,
  get_possible_melds,
  get_possible_melds_summary,
} from "pkg";

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
type Board = TileId[];
type Deck = Record<TileId, Tile>;

export type Game = {
  deck: Deck;
  id: GameId;
  name: string;
  players: PlayerId[];
  round: {
    player_index: PlayerId;
  };
  score: Score;
  table: {
    board: Board;
    draw_wall: TileId[];
    hands: Hands;
  };
};

export type ServicePlayer = {
  id: string;
  name: string;
};
export type ServicePlayerSummary = {
  id: string;
  name: string;
};

export type ServiceGame = {
  game: Game;
  players: Record<PlayerId, ServicePlayer>;
};

export type GameSummary = {
  board: Board;
  deck: Deck;
  draw_wall_count: number;
  hand: Hand;
  id: GameId;
  players: PlayerId[];
  player_id: PlayerId;
  round: {
    player_index: number;
  };
  score: Score;
};

export type ServiceGameSummary = {
  game_summary: GameSummary;
  players: Record<PlayerId, ServicePlayerSummary>;
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
  GameSummaryUpdate: ServiceGameSummary;
};

export type TUserGetGamesQuery = {
  player_id: PlayerId;
};
export type TUserGetGamesResponse = GameId[];

export type TUserLoadGameQuery = {
  player_id: PlayerId;
};
export type TUserLoadGameResponse = ServiceGameSummary;

export type TUserPostDrawTileRequest = {
  player_id: PlayerId;
};
export type TUserPostDrawTileResponse = ServiceGameSummary;

export type TUserPostDiscardTileRequest = {
  player_id: PlayerId;
  tile_id: TileId;
};
export type TUserPostDiscardTileResponse = ServiceGameSummary;

export type TUserPostMovePlayerRequest = {
  player_id: PlayerId;
};
export type TUserPostMovePlayerResponse = ServiceGameSummary;

export type TUserPostSortHandRequest = {
  player_id: PlayerId;
};
export type TUserPostSortHandResponse = ServiceGameSummary;

export type TUserPostCreateMeldRequest = {
  player_id: PlayerId;
  tiles: TileId[];
};
export type TUserPostCreateMeldResponse = ServiceGameSummary;

export type TUserPostBreakMeldRequest = {
  player_id: PlayerId;
  set_id: SetId;
};
export type TUserPostBreakMeldResponse = ServiceGameSummary;

export type TSocketQuery = {
  game_id: GameId;
  player_id?: PlayerId;
};

export class ModelServiceGame {
  constructor(public data: ServiceGame) {}

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

export class ModelServiceGameSummary {
  constructor(public data: ServiceGameSummary) {}

  getTileString(tileId: TileId) {
    const tile = this.data.game_summary.deck[tileId];
    const tileString = format_tile(tile);

    return `[${tileString}]`;
  }

  getPossibleMelds(): PossibleMeld[] {
    const possibleMelds = get_possible_melds_summary(JSON.stringify(this.data));

    return possibleMelds;
  }
}
