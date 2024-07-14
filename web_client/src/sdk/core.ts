import type { GameSettings } from "bindings/GameSettings";
import type { Hand } from "bindings/Hand";
import type { HandTile } from "bindings/HandTile";
import type { ServiceGameSummary } from "bindings/ServiceGameSummary";
import type { ServicePlayerGame } from "bindings/ServicePlayerGame";
import type { Tile } from "bindings/Tile";
import type { UserGetInfoResponse } from "bindings/UserGetInfoResponse";

export type GameId = string;
export type GameVersion = string;
export type PlayerId = string;
export type TileId = number;

export type SetId = string;

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
  createdAt: string;
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
      GameSummaryUpdate: ServiceGameSummary;
    }
  | {
      GameUpdate: ServiceGame;
    };

export type TSocketMessageFromClient = {
  type: "GetDeck";
};

export type TSocketWrapper = {
  close: () => void;
  send: (message: TSocketMessageFromClient) => void;
};

export type TUserGetGamesResponse = ServicePlayerGame[];

export type TUserPatchInfoResponse = UserGetInfoResponse;

export type TUserPostPassRoundResponse = ServiceGameSummary;

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

export type TUserPostMovePlayerResponse = ServiceGameSummary;

export type TUserPostSayMahjongRequest = {
  player_id: PlayerId;
};
export type TUserPostSayMahjongResponse = ServiceGameSummary;

export type TUserPostSetGameSettingsResponse = ServiceGameSummary;

export type TUserPostSortHandResponse = ServiceGameSummary;

export type TUserPostCreateMeldResponse = ServiceGameSummary;

export type TUserPostCreateGameRequest = {
  ai_player_names?: string[];
  player_id: PlayerId;
};
export type TUserPostCreateGameResponse = ServiceGameSummary;

export type TUserPostBreakMeldResponse = ServiceGameSummary;

export type TUserPostSetAuthRequest = {
  password: string;
  username: string;
};

export type TUserPostSetAnonAuthRequest = {
  id_token: string;
};
export type TUserPostSetAnonAuthResponse = {
  token: string;
};

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

export enum UserRole {
  Admin = "Admin",
  Player = "Player",
}

export type TokenClaims = {
  exp: number;
  role: UserRole;
  sub: PlayerId;
};

export enum AuthProvider {
  Anonymous = "Anonymous",
  Email = "Email",
  Github = "Github",
}

export type TTestDeleteGamesResponse = {
  test_delete_games: boolean;
};
