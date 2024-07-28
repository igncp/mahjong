import type { AdminPostMovePlayerResponse } from "bindings/AdminPostMovePlayerResponse";
import type { AdminPostSortHandsResponse } from "bindings/AdminPostSortHandsResponse";
import type { GetDeckResponse } from "bindings/GetDeckResponse";
import type { UserGetDashboardResponse } from "bindings/UserGetDashboardResponse";
import type { UserGetGamesQuery } from "bindings/UserGetGamesQuery";
import type { UserGetGamesResponse } from "bindings/UserGetGamesResponse";
import type { UserGetInfoResponse } from "bindings/UserGetInfoResponse";
import type { UserGetLoadGameResponse } from "bindings/UserGetLoadGameResponse";
import type { UserLoadGameQuery } from "bindings/UserLoadGameQuery";
import type { UserPatchInfoRequest } from "bindings/UserPatchInfoRequest";
import type { UserPatchInfoResponse } from "bindings/UserPatchInfoResponse";
import type { UserPostAIContinueRequest } from "bindings/UserPostAIContinueRequest";
import type { UserPostAIContinueResponse } from "bindings/UserPostAIContinueResponse";
import type { UserPostBreakMeldRequest } from "bindings/UserPostBreakMeldRequest";
import type { UserPostBreakMeldResponse } from "bindings/UserPostBreakMeldResponse";
import type { UserPostClaimTileRequest } from "bindings/UserPostClaimTileRequest";
import type { UserPostClaimTileResponse } from "bindings/UserPostClaimTileResponse";
import type { UserPostCreateGameRequest } from "bindings/UserPostCreateGameRequest";
import type { UserPostCreateGameResponse } from "bindings/UserPostCreateGameResponse";
import type { UserPostCreateMeldRequest } from "bindings/UserPostCreateMeldRequest";
import type { UserPostCreateMeldResponse } from "bindings/UserPostCreateMeldResponse";
import type { UserPostDiscardTileRequest } from "bindings/UserPostDiscardTileRequest";
import type { UserPostDiscardTileResponse } from "bindings/UserPostDiscardTileResponse";
import type { UserPostDrawTileRequest } from "bindings/UserPostDrawTileRequest";
import type { UserPostDrawTileResponse } from "bindings/UserPostDrawTileResponse";
import type { UserPostJoinGameResponse } from "bindings/UserPostJoinGameResponse";
import type { UserPostMovePlayerRequest } from "bindings/UserPostMovePlayerRequest";
import type { UserPostMovePlayerResponse } from "bindings/UserPostMovePlayerResponse";
import type { UserPostPassRoundRequest } from "bindings/UserPostPassRoundRequest";
import type { UserPostPassRoundResponse } from "bindings/UserPostPassRoundResponse";
import type { UserPostSayMahjongRequest } from "bindings/UserPostSayMahjongRequest";
import type { UserPostSayMahjongResponse } from "bindings/UserPostSayMahjongResponse";
import type { UserPostSetAuthAnonRequest } from "bindings/UserPostSetAuthAnonRequest";
import type { UserPostSetAuthAnonResponse } from "bindings/UserPostSetAuthAnonResponse";
import type { UserPostSetAuthRequest } from "bindings/UserPostSetAuthRequest";
import type { UserPostSetAuthResponse } from "bindings/UserPostSetAuthResponse";
import type { UserPostSetGameSettingsRequest } from "bindings/UserPostSetGameSettingsRequest";
import type { UserPostSetGameSettingsResponse } from "bindings/UserPostSetGameSettingsResponse";
import type { UserPostSortHandRequest } from "bindings/UserPostSortHandRequest";
import type { UserPostSortHandResponse } from "bindings/UserPostSortHandResponse";
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
  TAdminPostNewGameResponse,
  TAdminPostSayMahjongRequest,
  TAdminPostSayMahjongResponse,
  TSocketMessageFromClient,
  TSocketMessageFromServer,
  TSocketWrapper,
  TTestDeleteGamesResponse,
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
    opts: TAdminPostAIContinueRequest = {},
  ): Promise<TAdminPostAIContinueResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/ai-continue`, {
      body: JSON.stringify(opts),
      method: "POST",
    });
  },

  async adminBreakMeld(
    gameId: GameId,
    body: TAdminPostBreakMeldRequest,
  ): Promise<TAdminPostBreakMeldResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/break-meld`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async adminCreateMeld(
    gameId: GameId,
    body: TAdminPostCreateMeldRequest,
  ): Promise<TAdminPostCreateMeldResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/create-meld`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async adminDiscardTile(
    gameId: GameId,
    body: TAdminPostDiscardTileRequest,
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

  adminGetGame(gameId: GameId) {
    return from(fetchJson<TAdminGetGameResponse>(`/v1/admin/game/${gameId}`));
  },

  adminGetGames() {
    return from(fetchJson<TAdminGetGamesResponse>("/v1/admin/game"));
  },

  async adminMovePlayer(gameId: GameId): Promise<AdminPostMovePlayerResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/move-player`, {
      method: "POST",
    });
  },

  adminNewGame() {
    return from(
      fetchJson<TAdminPostNewGameResponse>(`/v1/admin/game`, {
        method: "POST",
      }),
    );
  },

  async adminSayMahjong(
    gameId: GameId,
    body: TAdminPostSayMahjongRequest,
  ): Promise<TAdminPostSayMahjongResponse> {
    return await fetchJson(`/v1/admin/game/${gameId}/say-mahjong`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  },

  async adminSortHands(gameId: GameId): Promise<AdminPostSortHandsResponse> {
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
    return from(fetchJson<GetDeckResponse>(`/v1/deck`));
  },

  getHealth: async (): Promise<void> =>
    await fetch(`${baseUrl}/health`).then(() => undefined),

  getUserDashboard() {
    return from(
      fetchJson<UserGetDashboardResponse>(`/v1/user/dashboard`, {
        method: "GET",
      }),
    );
  },

  setAuth(body: UserPostSetAuthRequest) {
    return from(
      fetchJson<UserPostSetAuthResponse>("/v1/user", {
        body: JSON.stringify(body),
        method: "POST",
      }),
    );
  },

  setAuthAnonymous(body: UserPostSetAuthAnonRequest) {
    return from(
      fetchJson<UserPostSetAuthAnonResponse>("/v1/user/anonymous", {
        body: JSON.stringify(body),
        method: "POST",
      }),
    );
  },

  testDeleteGames() {
    return from(
      fetchJson<TTestDeleteGamesResponse>(`/v1/test/delete-games`, {
        method: "POST",
      }),
    );
  },

  userBreakMeld(gameId: GameId, body: UserPostBreakMeldRequest) {
    return from(
      fetchJson<UserPostBreakMeldResponse>(
        `/v1/user/game/${gameId}/break-meld`,
        {
          body: JSON.stringify(body),
          method: "POST",
        },
      ),
    );
  },

  userClaimTile(gameId: GameId, body: UserPostClaimTileRequest) {
    return from(
      fetchJson<UserPostClaimTileResponse>(
        `/v1/user/game/${gameId}/claim-tile`,
        {
          body: JSON.stringify(body),
          method: "POST",
        },
      ),
    );
  },

  userContinueAI(gameId: GameId, body: UserPostAIContinueRequest) {
    return from(
      fetchJson<UserPostAIContinueResponse>(
        `/v1/user/game/${gameId}/ai-continue`,
        {
          body: JSON.stringify(body),
          method: "POST",
        },
      ),
    );
  },

  userCreateGame(body: UserPostCreateGameRequest) {
    return from(
      fetchJson<UserPostCreateGameResponse>("/v1/user/game", {
        body: JSON.stringify(body),
        method: "POST",
      }),
    );
  },

  userCreateMeld(gameId: GameId, body: UserPostCreateMeldRequest) {
    return from(
      fetchJson<UserPostCreateMeldResponse>(
        `/v1/user/game/${gameId}/create-meld`,
        {
          body: JSON.stringify(body),
          method: "POST",
        },
      ),
    );
  },

  userDiscardTile(gameId: GameId, body: UserPostDiscardTileRequest) {
    return from(
      fetchJson<UserPostDiscardTileResponse>(
        `/v1/user/game/${gameId}/discard-tile`,
        {
          body: JSON.stringify(body),
          method: "POST",
        },
      ),
    );
  },

  userDrawTile(gameId: GameId, body: UserPostDrawTileRequest) {
    return from(
      fetchJson<UserPostDrawTileResponse>(`/v1/user/game/${gameId}/draw-tile`, {
        body: JSON.stringify(body),
        method: "POST",
      }),
    );
  },

  userGetGames(query: UserGetGamesQuery) {
    return from(
      fetchJson<UserGetGamesResponse>(`/v1/user/game?${qs.stringify(query)}`),
    );
  },

  userGetInfo(userId: PlayerId) {
    return from(fetchJson<UserGetInfoResponse>(`/v1/user/info/${userId}`));
  },

  userJoinGame(gameId: GameId) {
    return from(
      fetchJson<UserPostJoinGameResponse>(`/v1/user/game/${gameId}/join`, {
        method: "POST",
      }),
    );
  },

  userLoadGame(gameId: GameId, query: UserLoadGameQuery) {
    return from(
      fetchJson<UserGetLoadGameResponse>(
        `/v1/user/game/${gameId}?${qs.stringify(query)}`,
      ),
    );
  },

  userMovePlayer(gameId: GameId, body: UserPostMovePlayerRequest) {
    return from(
      fetchJson<UserPostMovePlayerResponse>(
        `/v1/user/game/${gameId}/move-player`,
        {
          body: JSON.stringify(body),
          method: "POST",
        },
      ),
    );
  },

  userPassRound(gameId: GameId, body: UserPostPassRoundRequest) {
    return from(
      fetchJson<UserPostPassRoundResponse>(
        `/v1/user/game/${gameId}/pass-round`,
        {
          body: JSON.stringify(body),
          method: "POST",
        },
      ),
    );
  },

  userPatchInfo(userId: PlayerId, body: UserPatchInfoRequest) {
    return from(
      fetchJson<UserPatchInfoResponse>(`/v1/user/info/${userId}`, {
        body: JSON.stringify(body),
        method: "PATCH",
      }),
    );
  },

  userSayMahjong(gameId: GameId, body: UserPostSayMahjongRequest) {
    return from(
      fetchJson<UserPostSayMahjongResponse>(
        `/v1/user/game/${gameId}/say-mahjong`,
        {
          body: JSON.stringify(body),
          method: "POST",
        },
      ),
    );
  },

  userSetGameSettings(gameId: GameId, body: UserPostSetGameSettingsRequest) {
    return from(
      fetchJson<UserPostSetGameSettingsResponse>(
        `/v1/user/game/${gameId}/settings`,
        {
          body: JSON.stringify(body),
          method: "POST",
        },
      ),
    );
  },

  userSortHand(gameId: GameId, body: UserPostSortHandRequest) {
    return from(
      fetchJson<UserPostSortHandResponse>(`/v1/user/game/${gameId}/sort-hand`, {
        body: JSON.stringify(body),
        method: "POST",
      }),
    );
  },
};
