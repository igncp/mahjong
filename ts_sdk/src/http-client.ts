import { request } from "graphql-request";
import qs from "qs";
import { BehaviorSubject, Observable, from } from "rxjs";

import { getAuthTokenHeader, tokenObserver } from "./auth";
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
  TGetDeckResponse,
  TSocketMessageFromClient,
  TSocketMessageFromServer,
  TSocketQuery,
  TSocketWrapper,
  TUserGetGamesQuery,
  TUserGetGamesResponse,
  TUserGetInfoResponse,
  TUserLoadGameQuery,
  TUserLoadGameResponse,
  TUserPatchInfoRequest,
  TUserPatchInfoResponse,
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
  TUserPostSetGameSettingsRequest,
  TUserPostSetGameSettingsResponse,
  TUserPostSortHandRequest,
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
    playerId?: PlayerId;
    onMessage: (message: TSocketMessageFromServer) => void;
  }) {
    const { gameId, playerId, onMessage } = opts;
    let isIntentional = false;
    const query: TSocketQuery = {
      game_id: gameId,
      token: tokenObserver.getValue() as string,
      ...(playerId && { player_id: playerId }),
    };

    const sockerUrl = baseUrl.replace("https", "wss").replace("http", "ws");
    const socket = new WebSocket(`${sockerUrl}/v1/ws?${qs.stringify(query)}`);

    socket.onmessage = (event) => {
      const data: TSocketMessageFromServer = JSON.parse(event.data);
      onMessage(data);
    };

    socket.onerror = (error) => {
      console.log("Socket onerrror", error);
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

  fetchGraphQLQuery<T>(document: string): Observable<T> {
    return from(
      request<T>({
        url: `${baseUrl}/v1/graphql`,
        document,
        requestHeaders: {
          ...getAuthTokenHeader(),
          "Content-Type": "application/json",
        },
      })
    );
  },

  getDeck() {
    return from(fetchJson<TGetDeckResponse>(`/v1/deck`));
  },

  getHealth: async (): Promise<void> =>
    await fetch(`${baseUrl}/health`).then(() => undefined),

  setAuth(body: TUserPostSetAuthRequest) {
    return from(
      fetchJson<TUserPostSetAuthResponse>("/v1/user", {
        body: JSON.stringify(body),
        method: "POST",
      })
    );
  },

  userBreakMeld(gameId: GameId, body: TUserPostBreakMeldRequest) {
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

  userCreateMeld(gameId: GameId, body: TUserPostCreateMeldRequest) {
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

  userGetGames(query: TUserGetGamesQuery) {
    return from(
      fetchJson<TUserGetGamesResponse>(`/v1/user/game?${qs.stringify(query)}`)
    );
  },

  userGetInfo(userId: PlayerId) {
    return from(fetchJson<TUserGetInfoResponse>(`/v1/user/info/${userId}`));
  },

  userLoadGame(gameId: GameId, query: TUserLoadGameQuery) {
    return from(
      fetchJson<TUserLoadGameResponse>(
        `/v1/user/game/${gameId}?${qs.stringify(query)}`
      )
    );
  },

  userMovePlayer(gameId: GameId, body: TUserPostMovePlayerRequest) {
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

  userPatchInfo(userId: PlayerId, body: TUserPatchInfoRequest) {
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

  userSetGameSettings(gameId: GameId, body: TUserPostSetGameSettingsRequest) {
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

  userSortHand(gameId: GameId, body: TUserPostSortHandRequest) {
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
