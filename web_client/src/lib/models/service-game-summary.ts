import { format_tile, get_possible_melds_summary } from "pkg";

import { HttpClient } from "../http-client";
import {
  GameSettings,
  PossibleMeld,
  ServiceGameSummary,
  TileId,
} from "../mahjong-service";

export type ModelState<A> = [A, (v: A) => void];

export class ModelServiceGameSummary {
  public isLoading = false;

  public gameState!: ModelState<ServiceGameSummary>;
  public loadingState!: ModelState<boolean>;

  public updateStates(
    gameState: ModelState<ServiceGameSummary>,
    loadingState: ModelState<boolean>
  ) {
    this.gameState = gameState;
    this.loadingState = loadingState;
  }

  async breakMeld(setId: string) {
    try {
      if (this.loadingState[0]) {
        return;
      }

      this.loadingState[1](true);

      const newGame = await HttpClient.userBreakMeld(
        this.gameState[0].game_summary.id,
        {
          player_id: this.gameState[0].game_summary.player_id,
          set_id: setId,
        }
      );

      this.loadingState[1](false);
      this.gameState[1](newGame);
    } catch {
      this.handleError();
    }
  }

  async claimTile() {
    try {
      if (this.loadingState[0]) {
        return;
      }

      this.loadingState[1](true);

      const newGame = await HttpClient.userClaimTile(
        this.gameState[0].game_summary.id,
        {
          player_id: this.gameState[0].game_summary.player_id,
        }
      );

      this.loadingState[1](false);
      this.gameState[1](newGame);
    } catch {
      this.handleError();
    }
  }

  async createMeld(tiles: TileId[]) {
    try {
      if (this.loadingState[0]) {
        return;
      }

      this.loadingState[1](true);

      const newGame = await HttpClient.userCreateMeld(
        this.gameState[0].game_summary.id,
        {
          player_id: this.gameState[0].game_summary.player_id,
          tiles,
        }
      );

      this.loadingState[1](false);
      this.gameState[1](newGame);
    } catch {
      this.handleError();
    }
  }

  async discardTile(tileId: TileId) {
    try {
      if (this.loadingState[0]) {
        return;
      }

      this.loadingState[1](true);

      const serviceGame = await HttpClient.userDiscardTile(
        this.gameState[0].game_summary.id,
        {
          player_id: this.gameState[0].game_summary.player_id,
          tile_id: tileId,
        }
      );

      this.loadingState[1](false);
      this.gameState[1](serviceGame);
    } catch {
      this.handleError();
    }
  }

  getPlayingPlayer() {
    return this.gameState[0].players[this.gameState[0].game_summary.player_id];
  }

  getPlayingPlayerIndex() {
    return this.gameState[0].game_summary.players.findIndex(
      (player) => player === this.gameState[0].game_summary.player_id
    );
  }

  getTileString(tileId: TileId) {
    try {
      const tile = this.gameState[0].game_summary.deck[tileId];
      const tileString = format_tile(tile);

      return `[${tileString}]`;
    } catch (err) {
      console.log("debug: service-game-summary.ts: err", err);
    }
    return "";
  }

  getPossibleMelds(): PossibleMeld[] {
    try {
      const possibleMelds = get_possible_melds_summary(
        JSON.stringify(this.gameState[0])
      );

      return possibleMelds;
    } catch (error) {
      console.log("debug: service-game-summary.ts: error", error);
    }

    return [];
  }

  async sayMahjong() {
    try {
      if (this.loadingState[0]) {
        return;
      }

      this.loadingState[1](true);

      const newGame = await HttpClient.userSayMahjong(
        this.gameState[0].game_summary.id,
        {
          player_id: this.gameState[0].game_summary.player_id,
        }
      );

      this.loadingState[1](false);
      this.gameState[1](newGame);
    } catch {
      this.handleError();
    }
  }

  async sortHands() {
    try {
      if (this.loadingState[0]) {
        return;
      }

      this.loadingState[1](true);

      const newGame = await HttpClient.userSortHand(
        this.gameState[0].game_summary.id,
        {
          game_version: this.gameState[0].game_summary.version,
          player_id: this.gameState[0].game_summary.player_id,
        }
      );

      this.loadingState[1](false);
      this.gameState[1](newGame);
    } catch {
      this.handleError();
    }
  }

  async setGameSettings(gameSettings: GameSettings) {
    try {
      if (this.loadingState[0]) {
        return;
      }

      this.loadingState[1](true);

      await HttpClient.userSetGameSettings(this.gameState[0].game_summary.id, {
        player_id: this.gameState[0].game_summary.player_id,
        settings: gameSettings,
      });

      this.loadingState[1](false);

      this.gameState[1]({
        ...this.gameState[0],
        settings: gameSettings,
      });
    } catch (e) {
      console.log("ERR", e);
      this.handleError();
    }
  }

  getTile(tileId: TileId) {
    return this.gameState[0].game_summary.deck[tileId];
  }

  private handleError = () => {
    this.loadingState[1](false);
  };
}
