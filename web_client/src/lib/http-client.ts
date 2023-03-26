import { env } from "./env";
import {
  GameId,
  TAdminGetGameResponse,
  TAdminGetGamesResponse,
  TAdminPostNewGameResponse,
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
    return await fetch(`${this.baseUrl}${url}`, opts).then((r) => r.json());
  }

  public async getHealth(): Promise<void> {
    return await fetch(`${this.baseUrl}/health`).then(() => undefined);
  }

  public async adminGetGames(): Promise<TAdminGetGamesResponse> {
    return await this.fetchJson("/v1/admin/game");
  }

  public async adminGetGame(gameId: GameId): Promise<TAdminGetGameResponse> {
    return await this.fetchJson(`/v1/admin/game/${gameId}`);
  }

  public async adminNewGame(): Promise<TAdminPostNewGameResponse> {
    return await this.fetchJson(`/v1/admin/game`, { method: "POST" });
  }
}
