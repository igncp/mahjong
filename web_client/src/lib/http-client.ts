import qs from "qs";

import { env } from "./env";
import {
  GameId,
  TAdminGetGameResponse,
  TAdminGetGamesResponse,
  TAdminPostAIContinueRequest,
  TAdminPostAIContinueResponse,
  TAdminPostBreakMeldRequest,
  TAdminPostBreakMeldResponse,
  TAdminPostCreateMeldRequest,
  TAdminPostCreateMeldResponse,
  TAdminPostDiscardTileRequest,
  TAdminPostDiscardTileResponse,
  TAdminPostDrawCardResponse,
  TAdminPostDrawWallSwapTilesRequest,
  TAdminPostDrawWallSwapTilesResponse,
  TAdminPostMovePlayerResponse,
  TAdminPostNewGameResponse,
  TAdminPostSayMahjongRequest,
  TAdminPostSayMahjongResponse,
  TAdminPostSortHandsResponse,
  TSocketMessage,
  TUserGetGamesQuery,
  TUserGetGamesResponse,
  TUserLoadGameQuery,
  TUserLoadGameResponse,
} from "./mahjong-service";

const baseUrl = env.SERVICE_URL;

const fetchJson = <T>(url: string, opts?: RequestInit): Promise<T> =>
  fetch(`${baseUrl}${url}`, {
    ...opts,
    headers: {
      "Content-Type": "application/json",
      ...opts?.headers,
    },
  }).then((r) => r.json());

export const HttpClient = {
  async adminAIContinue(
    gameId: GameId,
    opts: TAdminPostAIContinueRequest = {}
  ): Promise<TAdminPostAIContinueResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/ai-continue`, {
      body: JSON.stringify(opts),
      method: "POST",
    });
  },

  async adminBreakMeld(
    gameId: GameId,
    body: TAdminPostBreakMeldRequest
  ): Promise<TAdminPostBreakMeldResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/break-meld`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async adminCreateMeld(
    gameId: GameId,
    body: TAdminPostCreateMeldRequest
  ): Promise<TAdminPostCreateMeldResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/create-meld`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async adminDiscardTile(
    gameId: GameId,
    body: TAdminPostDiscardTileRequest
  ): Promise<TAdminPostDiscardTileResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/discard-tile`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async adminDrawCard(gameId: GameId): Promise<TAdminPostDrawCardResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/draw-tile`, {
      method: "POST",
    });
  },

  async adminDrawWallSwapTiles(
    gameId: GameId,
    body: TAdminPostDrawWallSwapTilesRequest
  ): Promise<TAdminPostDrawWallSwapTilesResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/draw-wall-swap-tiles`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async adminGetGame(gameId: GameId): Promise<TAdminGetGameResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}`);
  },

  async adminGetGames(): Promise<TAdminGetGamesResponse> {
    return await fetchJson("/v1/admin/game");
  },

  async adminMovePlayer(gameId: GameId): Promise<TAdminPostMovePlayerResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/move-player`, {
      method: "POST",
    });
  },

  async adminNewGame(): Promise<TAdminPostNewGameResponse> {
    return await fetchJson(`/v1/admin/game`, { method: "POST" });
  },

  async adminSayMahjong(
    gameId: GameId,
    body: TAdminPostSayMahjongRequest
  ): Promise<TAdminPostSayMahjongResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/say-mahjong`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async adminSortHands(gameId: GameId): Promise<TAdminPostSortHandsResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/sort-hands`, {
      method: "POST",
    });
  },

  async connectToSocket({
    gameId,
    onMessage,
  }: {
    gameId: GameId;
    onMessage: (message: TSocketMessage) => void;
  }) {
    const socket = new WebSocket(
      `${baseUrl.replace("http", "ws")}/v1/ws?game_id=${gameId}`
    );

    socket.onmessage = (event) => {
      const data: TSocketMessage = JSON.parse(event.data);
      onMessage(data);
    };

    return () => {
      socket.close();
    };
  },

  getHealth: async (): Promise<void> =>
    await fetch(`${baseUrl}/health`).then(() => undefined),

  async userGetGames(
    query: TUserGetGamesQuery
  ): Promise<TUserGetGamesResponse> {
    return await fetchJson(`/v1/user/game?${qs.stringify(query)}`);
  },

  async userLoadGame(
    gameId: GameId,
    query: TUserLoadGameQuery
  ): Promise<TUserLoadGameResponse> {
    return await fetchJson(`/v1/user/game/${gameId}?${qs.stringify(query)}`);
  },
};
