import qs from "qs";

import { getAuthTokenHeader, tokenObserver } from "./auth";
import { env } from "./env";
import {
  GameId,
  PlayerId,
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
  TSocketQuery,
  TUserGetGamesQuery,
  TUserGetGamesResponse,
  TUserLoadGameQuery,
  TUserLoadGameResponse,
  TUserPostBreakMeldRequest,
  TUserPostBreakMeldResponse,
  TUserPostClaimTileRequest,
  TUserPostClaimTileResponse,
  TUserPostContinueAIRequest,
  TUserPostContinueAIResponse,
  TUserPostCreateGameRequest,
  TUserPostCreateGameResponse,
  TUserPostCreateMeldRequest,
  TUserPostCreateMeldResponse,
  TUserPostDiscardTileRequest,
  TUserPostDiscardTileResponse,
  TUserPostDrawTileRequest,
  TUserPostDrawTileResponse,
  TUserPostMovePlayerRequest,
  TUserPostMovePlayerResponse,
  TUserPostSayMahjongRequest,
  TUserPostSayMahjongResponse,
  TUserPostSetAuthRequest,
  TUserPostSetAuthResponse,
  TUserPostSetSettingsRequest,
  TUserPostSetSettingsResponse,
  TUserPostSortHandRequest,
  TUserPostSortHandResponse,
} from "./mahjong-service";

const baseUrl = env.SERVICE_URL;

const fetchJson = <T>(url: string, opts?: RequestInit): Promise<T> => {
  const tokenHeader = getAuthTokenHeader();

  return fetch(`${baseUrl}${url}`, {
    ...opts,
    headers: {
      "Content-Type": "application/json",
      ...tokenHeader,
      ...opts?.headers,
    },
  }).then((r) => r.json());
};

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

  async connectToSocket(opts: {
    gameId: GameId;
    playerId?: PlayerId;
    onMessage: (message: TSocketMessage) => void;
  }) {
    const { gameId, playerId, onMessage } = opts;
    let isIntentional = false;
    const query: TSocketQuery = {
      game_id: gameId,
      token: tokenObserver.getValue() as string,
      ...(playerId && { player_id: playerId }),
    };

    const socket = new WebSocket(
      `${baseUrl.replace("http", "ws")}/v1/ws?${qs.stringify(query)}`
    );

    socket.onmessage = (event) => {
      const data: TSocketMessage = JSON.parse(event.data);
      onMessage(data);
    };

    socket.onerror = () => {
      console.log("Socket onerrror");
    };

    socket.onclose = () => {
      if (!isIntentional) {
        setTimeout(() => {
          console.log("Trying to reconnect onclose");
          HttpClient.connectToSocket(opts);
        }, 1000);
      }
    };

    return () => {
      isIntentional = true;
      socket.close();
    };
  },

  getHealth: async (): Promise<void> =>
    await fetch(`${baseUrl}/health`).then(() => undefined),

  async setAuth(
    body: TUserPostSetAuthRequest
  ): Promise<TUserPostSetAuthResponse> {
    return await fetchJson("/v1/user", {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async userBreakMeld(
    gameId: GameId,
    body: TUserPostBreakMeldRequest
  ): Promise<TUserPostBreakMeldResponse> {
    return await fetchJson(`/v1/user/game/${gameId}/break-meld`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async userClaimTile(
    gameId: GameId,
    body: TUserPostClaimTileRequest
  ): Promise<TUserPostClaimTileResponse> {
    return await fetchJson(`/v1/user/game/${gameId}/claim-tile`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async userContinueAI(
    gameId: GameId,
    body: TUserPostContinueAIRequest
  ): Promise<TUserPostContinueAIResponse> {
    return await fetchJson(`/v1/user/game/${gameId}/ai-continue`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async userCreateGame(
    body: TUserPostCreateGameRequest
  ): Promise<TUserPostCreateGameResponse> {
    return await fetchJson("/v1/user/game", {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async userCreateMeld(
    gameId: GameId,
    body: TUserPostCreateMeldRequest
  ): Promise<TUserPostCreateMeldResponse> {
    return await fetchJson(`/v1/user/game/${gameId}/create-meld`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async userDiscardTile(
    gameId: GameId,
    body: TUserPostDiscardTileRequest
  ): Promise<TUserPostDiscardTileResponse> {
    return await fetchJson(`/v1/user/game/${gameId}/discard-tile`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async userDrawTile(
    gameId: GameId,
    body: TUserPostDrawTileRequest
  ): Promise<TUserPostDrawTileResponse> {
    return await fetchJson(`/v1/user/game/${gameId}/draw-tile`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

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

  async userMovePlayer(
    gameId: GameId,
    body: TUserPostMovePlayerRequest
  ): Promise<TUserPostMovePlayerResponse> {
    return await fetchJson(`/v1/user/game/${gameId}/move-player`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async userSayMahjong(
    gameId: GameId,
    body: TUserPostSayMahjongRequest
  ): Promise<TUserPostSayMahjongResponse> {
    return await fetchJson(`/v1/user/game/${gameId}/say-mahjong`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async userSetSettings(
    body: TUserPostSetSettingsRequest
  ): Promise<TUserPostSetSettingsResponse> {
    return await fetchJson(`/v1/user/settings`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async userSortHand(
    gameId: GameId,
    body: TUserPostSortHandRequest
  ): Promise<TUserPostSortHandResponse> {
    return await fetchJson(`/v1/user/game/${gameId}/sort-hand`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },
};
