import type { AdminPostMovePlayerResponse } from "bindings/AdminPostMovePlayerResponse";
import type { AdminPostSortHandsResponse } from "bindings/AdminPostSortHandsResponse";
import type { GetDeckResponse } from "bindings/GetDeckResponse";
import type { Queries } from "bindings/Queries";
import type { QueriesResponses } from "bindings/QueriesResponses";
import type { UserGetInfoResponse } from "bindings/UserGetInfoResponse";
import type { UserGetLoadGameResponse } from "bindings/UserGetLoadGameResponse";
import type { UserLoadGameQuery } from "bindings/UserLoadGameQuery";
import type { UserPatchInfoRequest } from "bindings/UserPatchInfoRequest";
import type { UserPatchInfoResponse } from "bindings/UserPatchInfoResponse";
import type { UserPostAIContinueRequest } from "bindings/UserPostAIContinueRequest";
import type { UserPostAIContinueResponse } from "bindings/UserPostAIContinueResponse";
import type { UserPostClaimTileRequest } from "bindings/UserPostClaimTileRequest";
import type { UserPostClaimTileResponse } from "bindings/UserPostClaimTileResponse";
import type { UserPostJoinGameResponse } from "bindings/UserPostJoinGameResponse";
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

type Query<A> = Extract<Queries, { type: A }>;

type QueryResponse<A> = Extract<QueriesResponses, { type: A }>;

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

const userCommon = <A>(body: Query<A>) =>
  from(
    fetchJson<QueryResponse<A>>("/v1/user/game", {
      body: JSON.stringify(body),
      method: "POST",
    }),
  );

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

  getUserDashboard: userCommon<"UserGetDashboard">,

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

  userBreakMeld: userCommon<"UserBreakMeld">,

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

  userCreateGame: userCommon<"UserCreateGame">,

  userCreateMeld: userCommon<"UserCreateMeld">,

  userDiscardTile: userCommon<"UserDiscardTile">,

  userDrawTile: userCommon<"UserDrawTile">,

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

  userMovePlayer: userCommon<"UserMovePlayer">,

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
