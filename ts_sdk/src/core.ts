// These are maintained manually. An alternative would be to use:
// https://github.com/Aleph-Alpha/ts-rs or OpenAPI spec but for now it is
// faster to maintain manually.

export type GameId = string;
export type GameVersion = string;
export type PlayerId = string;
export type TileId = number;

export enum Flower {
  Bamboo = "Bamboo",
  Chrysanthemum = "Chrysanthemum",
  Orchid = "Orchid",
  Plum = "Plum",
}

export enum Season {
  Autumn = "Autumn",
  Spring = "Spring",
  Summer = "Summer",
  Winter = "Winter",
}

export enum Wind {
  East = "East",
  North = "North",
  South = "South",
  West = "West",
}

export enum Dragon {
  Green = "Green",
  Red = "Red",
  White = "White",
}

export enum Suit {
  Bamboo = "Bamboo",
  Characters = "Characters",
  Dots = "Dots",
}

export type SuitTile = {
  Suit: {
    id: TileId;
    suit: Suit;
    value: number;
  };
};

export type FlowerTile = {
  Flower: {
    id: TileId;
    value: Flower;
  };
};

export type SeasonTile = {
  Season: {
    id: TileId;
    value: Season;
  };
};

export type WindTile = {
  Wind: {
    id: TileId;
    value: Wind;
  };
};

export type DragonTile = {
  Dragon: {
    id: TileId;
    value: Dragon;
  };
};

export type Tile = FlowerTile | SeasonTile | WindTile | DragonTile | SuitTile;
export type SetId = string;

export type ServicePlayerGame = {
  createdAt: string;
  id: GameId;
  updatedAt: string;
};

type HandTile = {
  concealed: boolean;
  id: TileId;
  set_id: SetId;
};
export type Hand = HandTile[];
export type Hands = Record<PlayerId, Hand>;
export type Score = Record<PlayerId, number>;
export type Board = TileId[];
export type Deck = Map<TileId, Tile>;

export type Game = {
  id: GameId;
  name: string;
  players: PlayerId[];
  round: {
    player_index: number;
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
  settings: GameSettings;
};

export type HandSummary = {
  tiles: number;
  visible: HandTile[];
};

export type GameSettings = {
  ai_enabled: boolean;
  discard_wait_ms: number | null;
  fixed_settings: boolean;
  last_discard_time: string | null;
};

export type GameSettingsSummary = {
  ai_enabled: boolean;
  auto_sort: boolean;
  discard_wait_ms: number | null;
  fixed_settings: boolean;
};

export type GameSummary = {
  board: Board;
  draw_wall_count: number;
  hand: Hand;
  id: GameId;
  other_hands: Record<PlayerId, HandSummary>;
  players: PlayerId[];
  player_id: PlayerId;
  round: {
    dealer_player_index: number;
    discarded_tile: TileId | null;
    player_index: number;
    wind: Wind;
  };
  score: Score;
  version: GameVersion;
};

export type ServiceGameSummary = {
  settings: GameSettingsSummary;
  game_summary: GameSummary;
  players: Record<PlayerId, ServicePlayerSummary>;
};

export type TGetDeckRequest = void;
export type TGetDeckResponse = Deck;

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

export type TAdminGetGamesResponse = ServicePlayerGame[];
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

export type TSocketMessageFromServer =
  | {
      GameUpdate: ServiceGame;
    }
  | {
      GameSummaryUpdate: ServiceGameSummary;
    };

export type TSocketMessageFromClient = {
  type: "GetDeck";
};

export type TSocketWrapper = {
  close: () => void;
  send: (message: TSocketMessageFromClient) => void;
};

export type TUserGetGamesQuery = {
  player_id: PlayerId;
};
export type TUserGetGamesResponse = ServicePlayerGame[];

export type TUserGetInfoResponse = {
  name: string;
  total_score: number;
};
export type TUserPatchInfoRequest = {
  name: string;
};
export type TUserPatchInfoResponse = TUserGetInfoResponse;

export type TUserLoadGameQuery = {
  player_id: PlayerId;
};
export type TUserLoadGameResponse = ServiceGameSummary;

export type TUserPostDrawTileRequest = {
  game_version: GameVersion;
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

export type TUserPostSayMahjongRequest = {
  player_id: PlayerId;
};
export type TUserPostSayMahjongResponse = ServiceGameSummary;

export type TUserPostSetGameSettingsRequest = {
  player_id: PlayerId;
  settings: GameSettingsSummary;
};
export type TUserPostSetGameSettingsResponse = ServiceGameSummary;

export type TUserPostSortHandRequest = {
  game_version: GameVersion;
  player_id: PlayerId;
  tiles?: TileId[];
};
export type TUserPostSortHandResponse = ServiceGameSummary;

export type TUserPostCreateMeldRequest = {
  player_id: PlayerId;
  tiles: TileId[];
};
export type TUserPostCreateMeldResponse = ServiceGameSummary;

export type TUserPostCreateGameRequest = {
  ai_player_names?: string[];
  player_id: PlayerId;
};
export type TUserPostCreateGameResponse = ServiceGameSummary;

export type TUserPostBreakMeldRequest = {
  player_id: PlayerId;
  set_id: SetId;
};
export type TUserPostBreakMeldResponse = ServiceGameSummary;

export type TUserPostSetAuthRequest = {
  password: string;
  username: string;
};
export type TUserPostSetAuthResponse =
  | {
      token: string;
    }
  | "E_INVALID_USER_PASS";

export type TUserPostContinueAIRequest = {
  player_id: PlayerId;
};
export type TUserPostContinueAIResponse = {
  changed: boolean;
  service_game_summary: ServiceGameSummary;
};

export type TUserPostClaimTileRequest = {
  player_id: PlayerId;
};
export type TUserPostClaimTileResponse = ServiceGameSummary;

export type TSocketQuery = {
  game_id: GameId;
  player_id?: PlayerId;
  token: string;
};

export enum UserRole {
  Player = "Player",
  Admin = "Admin",
}

export type TokenClaims = {
  exp: number;
  role: UserRole;
  sub: PlayerId;
};
