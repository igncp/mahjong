import type { UserGetDashboardResponse } from "bindings/UserGetDashboardResponse";
import type { UserGetGamesQuery } from "bindings/UserGetGamesQuery";
import type { UserGetInfoResponse } from "bindings/UserGetInfoResponse";
import type { UserPatchInfoRequest } from "bindings/UserPatchInfoRequest";
import type { UserPostBreakMeldRequest } from "bindings/UserPostBreakMeldRequest";
import type { UserPostCreateMeldRequest } from "bindings/UserPostCreateMeldRequest";
import type { UserPostMovePlayerRequest } from "bindings/UserPostMovePlayerRequest";
import type { UserPostPassRoundRequest } from "bindings/UserPostPassRoundRequest";
import type { UserPostSetAuthResponse } from "bindings/UserPostSetAuthResponse";
import type { UserPostSetGameSettingsRequest } from "bindings/UserPostSetGameSettingsRequest";
import type { UserPostSortHandRequest } from "bindings/UserPostSortHandRequest";
import type { WebSocketQuery } from "bindings/WebSocketQuery";
import qs from "qs";
import { BehaviorSubject, from } from "rxjs";

import { getAuthTokenHeader, tokenObserver } from "./auth";
import type {
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
  TGetDeckResponse,
  TSocketMessageFromClient,
  TSocketMessageFromServer,
  TSocketWrapper,
  TTestDeleteGamesResponse,
  TUserGetGamesResponse,
  TUserLoadGameQuery,
  TUserLoadGameResponse,
  TUserPatchInfoResponse,
  TUserPostBreakMeldResponse,
  TUserPostClaimTileRequest,
  TUserPostClaimTileResponse,
  TUserPostContinueAIRequest,
  TUserPostContinueAIResponse,
  TUserPostCreateGameRequest,
  TUserPostCreateGameResponse,
  TUserPostCreateMeldResponse,
  TUserPostDiscardTileRequest,
  TUserPostDiscardTileResponse,
  TUserPostDrawTileRequest,
  TUserPostDrawTileResponse,
  TUserPostMovePlayerResponse,
  TUserPostPassRoundResponse,
  TUserPostSayMahjongRequest,
  TUserPostSayMahjongResponse,
  TUserPostSetAnonAuthRequest,
  TUserPostSetAnonAuthResponse,
  TUserPostSetAuthRequest,
  TUserPostSetGameSettingsResponse,
  TUserPostSortHandResponse,
} from "./core";

let baseUrl = "";

export const setBaseUrl = (val: string) => {
  baseUrl = val;
};

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

  adminGetGame(gameId: GameId) {
    return from(fetchJson<TAdminGetGameResponse>(`/v1/admin/game/${gameId}`));
  },

  adminGetGames() {
    return from(fetchJson<TAdminGetGamesResponse>("/v1/admin/game"));
  },

  async adminMovePlayer(gameId: GameId): Promise<TAdminPostMovePlayerResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/move-player`, {
      method: "POST",
    });
  },

  adminNewGame() {
    return from(
      fetchJson<TAdminPostNewGameResponse>(`/v1/admin/game`, {
        method: "POST",
      })
    );
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

  connectToSocket(opts: {
    gameId: GameId;
    onMessage: (message: TSocketMessageFromServer) => void;
    playerId?: PlayerId;
  }) {
    const { gameId, onMessage, playerId } = opts;

    let isIntentional = false;

    const query: WebSocketQuery = {
      game_id: gameId,
      player_id: playerId || null,
      token: tokenObserver.getValue() as string,
    };

    const sockerUrl = baseUrl.replace("https", "wss").replace("http", "ws");
    const socket = new WebSocket(`${sockerUrl}/v1/ws?${qs.stringify(query)}`);

    socket.onmessage = (event) => {
      const data: TSocketMessageFromServer = JSON.parse(event.data);

      onMessage(data);
    };

    socket.onerror = (error) => {
      console.error("Socket onerrror", error);
    };

    let retryUnsubscribe = () => {};

    const socketWrapper: TSocketWrapper = {
      close: () => {
        isIntentional = true;
        retryUnsubscribe();
        socket.close();
      },
      send: (message: TSocketMessageFromClient) => {
        socket.send(JSON.stringify(message));
      },
    };

    const socketProvider = new BehaviorSubject<TSocketWrapper>(socketWrapper);

    socket.onclose = () => {
      if (!isIntentional) {
        setTimeout(() => {
          // eslint-disable-next-line no-console
          console.log("Trying to reconnect onclose");

          const subscription = HttpClient.connectToSocket(opts).subscribe({
            next: (newSocketWrapper) => {
              socketProvider.next(newSocketWrapper);
            },
          });

          retryUnsubscribe = () => subscription.unsubscribe();
        }, 10_000);
      }
    };

    return socketProvider;
  },

  getDeck() {
    return from(fetchJson<TGetDeckResponse>(`/v1/deck`));
  },

  getHealth: async (): Promise<void> =>
    await fetch(`${baseUrl}/health`).then(() => undefined),

  getUserDashboard() {
    return from(
      fetchJson<UserGetDashboardResponse>(`/v1/user/dashboard`, {
        method: "GET",
      })
    );
  },

  setAuth(body: TUserPostSetAuthRequest) {
    return from(
      fetchJson<UserPostSetAuthResponse>("/v1/user", {
        body: JSON.stringify(body),
        method: "POST",
      })
    );
  },

  setAuthAnonymous(body: TUserPostSetAnonAuthRequest) {
    return from(
      fetchJson<TUserPostSetAnonAuthResponse>("/v1/user-anonymous", {
        body: JSON.stringify(body),
        method: "POST",
      })
    );
  },

  testDeleteGames() {
    return from(
      fetchJson<TTestDeleteGamesResponse>(`/v1/test/delete-games`, {
        method: "POST",
      })
    );
  },

  userBreakMeld(gameId: GameId, body: UserPostBreakMeldRequest) {
    return from(
      fetchJson<TUserPostBreakMeldResponse>(
        `/v1/user/game/${gameId}/break-meld`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },

  userClaimTile(gameId: GameId, body: TUserPostClaimTileRequest) {
    return from(
      fetchJson<TUserPostClaimTileResponse>(
        `/v1/user/game/${gameId}/claim-tile`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },

  userContinueAI(gameId: GameId, body: TUserPostContinueAIRequest) {
    return from(
      fetchJson<TUserPostContinueAIResponse>(
        `/v1/user/game/${gameId}/ai-continue`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },

  userCreateGame(body: TUserPostCreateGameRequest) {
    return from(
      fetchJson<TUserPostCreateGameResponse>("/v1/user/game", {
        body: JSON.stringify(body),
        method: "POST",
      })
    );
  },

  userCreateMeld(gameId: GameId, body: UserPostCreateMeldRequest) {
    return from(
      fetchJson<TUserPostCreateMeldResponse>(
        `/v1/user/game/${gameId}/create-meld`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },

  userDiscardTile(gameId: GameId, body: TUserPostDiscardTileRequest) {
    return from(
      fetchJson<TUserPostDiscardTileResponse>(
        `/v1/user/game/${gameId}/discard-tile`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },

  userDrawTile(gameId: GameId, body: TUserPostDrawTileRequest) {
    return from(
      fetchJson<TUserPostDrawTileResponse>(
        `/v1/user/game/${gameId}/draw-tile`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },

  userGetGames(query: UserGetGamesQuery) {
    return from(
      fetchJson<TUserGetGamesResponse>(`/v1/user/game?${qs.stringify(query)}`)
    );
  },

  userGetInfo(userId: PlayerId) {
    return from(fetchJson<UserGetInfoResponse>(`/v1/user/info/${userId}`));
  },

  userLoadGame(gameId: GameId, query: TUserLoadGameQuery) {
    return from(
      fetchJson<TUserLoadGameResponse>(
        `/v1/user/game/${gameId}?${qs.stringify(query)}`
      )
    );
  },

  userMovePlayer(gameId: GameId, body: UserPostMovePlayerRequest) {
    return from(
      fetchJson<TUserPostMovePlayerResponse>(
        `/v1/user/game/${gameId}/move-player`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },

  userPassRound(gameId: GameId, body: UserPostPassRoundRequest) {
    return from(
      fetchJson<TUserPostPassRoundResponse>(
        `/v1/user/game/${gameId}/pass-round`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },

  userPatchInfo(userId: PlayerId, body: UserPatchInfoRequest) {
    return from(
      fetchJson<TUserPatchInfoResponse>(`/v1/user/info/${userId}`, {
        body: JSON.stringify(body),
        method: "PATCH",
      })
    );
  },

  userSayMahjong(gameId: GameId, body: TUserPostSayMahjongRequest) {
    return from(
      fetchJson<TUserPostSayMahjongResponse>(
        `/v1/user/game/${gameId}/say-mahjong`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },

  userSetGameSettings(gameId: GameId, body: UserPostSetGameSettingsRequest) {
    return from(
      fetchJson<TUserPostSetGameSettingsResponse>(
        `/v1/user/game/${gameId}/settings`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },

  userSortHand(gameId: GameId, body: UserPostSortHandRequest) {
    return from(
      fetchJson<TUserPostSortHandResponse>(
        `/v1/user/game/${gameId}/sort-hand`,
        {
          body: JSON.stringify(body),
          method: "POST",
        }
      )
    );
  },
};
