import type { GameSummary } from "bindings/GameSummary";
import type { Hand } from "bindings/Hand";
import type { HandTile } from "bindings/HandTile";
import type { ServiceGame } from "bindings/ServiceGame";
import type { ServiceGameSummary } from "bindings/ServiceGameSummary";
import type { ServicePlayerGame } from "bindings/ServicePlayerGame";

export type GameId = GameSummary["id"];
export type GameVersion = GameSummary["version"];
export type PlayerId = GameSummary["players"][number];
export type TileId = HandTile["id"];
export type SetId = HandTile["set_id"];
export type SetIdContent = NonNullable<HandTile["set_id"]>;

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

export type TAdminPostDiscardTileRequest = {
  tile_id: TileId;
};
export type TAdminPostDiscardTileResponse = ServiceGame;

export type TAdminPostMovePlayerRequest = void;

export type TAdminPostSortHandsRequest = void;

export type TAdminPostSayMahjongRequest = {
  player_id: PlayerId;
};
export type TAdminPostSayMahjongResponse = ServiceGame;

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

export type TTestDeleteGamesResponse = {
  test_delete_games: boolean;
};
