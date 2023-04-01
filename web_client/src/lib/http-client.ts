import { env } from "./env";
import {
  GameId,
  TAdminGetGameResponse,
  TAdminGetGamesResponse,
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
} from "./mahjong-service";

export class HttpClient {
  private baseUrl: string;
  private static instance: HttpClient;

  public static singleton() {
    if (!HttpClient.instance) {
      HttpClient.instance = new HttpClient();
    }

    return HttpClient.instance;
  }

  private constructor() {
    this.baseUrl = env.SERVICE_URL;
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

  public async getHealth(): Promise<void> {
    return await fetch(`${this.baseUrl}/health`).then(() => undefined);
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
}
