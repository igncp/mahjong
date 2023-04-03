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
  TAdminPostSortHandsResponse,
  TSocketMessage,
} from "./mahjong-service";

export class HttpClient {
  private static instance: HttpClient;
  private baseUrl: string;

  private constructor() {
    this.baseUrl = env.SERVICE_URL;
  }

  public static singleton() {
    if (!HttpClient.instance) {
      HttpClient.instance = new HttpClient();
    }

    return HttpClient.instance;
  }

  public async connectToSocket({
    gameId,
    onMessage,
  }: {
    gameId: GameId;
    onMessage: (message: TSocketMessage) => void;
  }) {
    const socket = new WebSocket(
      `${this.baseUrl.replace("http", "ws")}/v1/ws?game_id=${gameId}`
    );

    socket.onmessage = (event) => {
      const data: TSocketMessage = JSON.parse(event.data);
      onMessage(data);
    };

    return () => {
      socket.close();
    };
  }

  public async getHealth(): Promise<void> {
    return await fetch(`${this.baseUrl}/health`).then(() => undefined);
  }

  public async adminAIContinue(
    gameId: GameId,
    opts: TAdminPostAIContinueRequest = {}
  ): Promise<TAdminPostAIContinueResponse> {
    return await this.fetchJson(`/v1/admin/game/${gameId}/ai-continue`, {
      body: JSON.stringify(opts),
      method: "POST",
    });
  }

  public async adminBreakMeld(
    gameId: GameId,
    body: TAdminPostBreakMeldRequest
  ): Promise<TAdminPostBreakMeldResponse> {
    return await this.fetchJson(`/v1/admin/game/${gameId}/break-meld`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  }

  public async adminCreateMeld(
    gameId: GameId,
    body: TAdminPostCreateMeldRequest
  ): Promise<TAdminPostCreateMeldResponse> {
    return await this.fetchJson(`/v1/admin/game/${gameId}/create-meld`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  }

  public async adminDiscardTile(
    gameId: GameId,
    body: TAdminPostDiscardTileRequest
  ): Promise<TAdminPostDiscardTileResponse> {
    return await this.fetchJson(`/v1/admin/game/${gameId}/discard-tile`, {
      body: JSON.stringify(body),
      method: "POST",
    });
  }

  public async adminDrawCard(
    gameId: GameId
  ): Promise<TAdminPostDrawCardResponse> {
    return await this.fetchJson(`/v1/admin/game/${gameId}/draw-tile`, {
      method: "POST",
    });
  }

  public async adminDrawWallSwapTiles(
    gameId: GameId,
    body: TAdminPostDrawWallSwapTilesRequest
  ): Promise<TAdminPostDrawWallSwapTilesResponse> {
    return await this.fetchJson(
      `/v1/admin/game/${gameId}/draw-wall-swap-tiles`,
      {
        body: JSON.stringify(body),
        method: "POST",
      }
    );
  }

  public async adminGetGames(): Promise<TAdminGetGamesResponse> {
    return await this.fetchJson("/v1/admin/game");
  }

  public async adminGetGame(gameId: GameId): Promise<TAdminGetGameResponse> {
    return await this.fetchJson(`/v1/admin/game/${gameId}`);
  }

  public async adminMovePlayer(
    gameId: GameId
  ): Promise<TAdminPostMovePlayerResponse> {
    return await this.fetchJson(`/v1/admin/game/${gameId}/move-player`, {
      method: "POST",
    });
  }

  public async adminNewGame(): Promise<TAdminPostNewGameResponse> {
    return await this.fetchJson(`/v1/admin/game`, { method: "POST" });
  }

  public async adminSortHands(
    gameId: GameId
  ): Promise<TAdminPostSortHandsResponse> {
    return await this.fetchJson(`/v1/admin/game/${gameId}/sort-hands`, {
      method: "POST",
    });
  }

  private async fetchJson<T>(url: string, opts?: RequestInit): Promise<T> {
    return await fetch(`${this.baseUrl}${url}`, {
      ...opts,
      headers: {
        "Content-Type": "application/json",
        ...opts?.headers,
      },
    }).then((r) => r.json());
  }
}
