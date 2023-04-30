import { first } from "rxjs";

import { format_tile, get_deck, get_possible_melds_summary } from "pkg";

import {
  GameSettings,
  PossibleMeld,
  ServiceGameSummary,
  TileId,
} from "mahjong_sdk/src/core";
import { HttpClient } from "mahjong_sdk/src/http-server";

export type ModelState<A> = [A, (v: A) => void];

export const DEFAULT_DECK = get_deck();

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

  breakMeld(setId: string) {
    if (this.loadingState[0]) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userBreakMeld(this.gameState[0].game_summary.id, {
      player_id: this.gameState[0].game_summary.player_id,
      set_id: setId,
    })
      .pipe(first())
      .subscribe({
        error: () => {
          this.handleError();
        },
        next: (newGame) => {
          this.loadingState[1](false);
          this.gameState[1](newGame);
        },
      });
  }

  claimTile() {
    if (this.loadingState[0]) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userClaimTile(this.gameState[0].game_summary.id, {
      player_id: this.gameState[0].game_summary.player_id,
    }).subscribe({
      error: () => {
        this.handleError();
      },
      next: (newGame) => {
        this.loadingState[1](false);
        this.gameState[1](newGame);
      },
    });
  }

  createMeld(tiles: TileId[]) {
    if (this.loadingState[0]) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userCreateMeld(this.gameState[0].game_summary.id, {
      player_id: this.gameState[0].game_summary.player_id,
      tiles,
    }).subscribe({
      error: () => {
        this.handleError();
      },
      next: (newGame) => {
        this.loadingState[1](false);
        this.gameState[1](newGame);
      },
    });
  }

  discardTile(tileId: TileId) {
    if (this.loadingState[0]) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userDiscardTile(this.gameState[0].game_summary.id, {
      player_id: this.gameState[0].game_summary.player_id,
      tile_id: tileId,
    }).subscribe({
      error: () => {
        this.handleError();
      },
      next: (serviceGame) => {
        this.loadingState[1](false);
        this.gameState[1](serviceGame);
      },
    });
  }

  getTurnPlayer() {
    const playerId =
      this.gameState[0].game_summary.players[
        this.gameState[0].game_summary.round.player_index
      ];

    return this.gameState[0].players[playerId];
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
      const tile = DEFAULT_DECK[tileId];
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

  sayMahjong() {
    if (this.loadingState[0]) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userSayMahjong(this.gameState[0].game_summary.id, {
      player_id: this.gameState[0].game_summary.player_id,
    }).subscribe({
      next: (newGame) => {
        this.loadingState[1](false);
        this.gameState[1](newGame);
      },
    });
  }

  sortHands() {
    if (this.loadingState[0]) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userSortHand(this.gameState[0].game_summary.id, {
      game_version: this.gameState[0].game_summary.version,
      player_id: this.gameState[0].game_summary.player_id,
    })
      .pipe(first())
      .subscribe({
        error: () => {
          this.handleError();
        },
        next: (newGame) => {
          this.loadingState[1](false);
          this.gameState[1](newGame);
        },
      });
  }

  setGameSettings(gameSettings: GameSettings) {
    if (this.loadingState[0]) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userSetGameSettings(this.gameState[0].game_summary.id, {
      player_id: this.gameState[0].game_summary.player_id,
      settings: gameSettings,
    }).subscribe({
      next: () => {
        this.loadingState[1](false);

        this.gameState[1]({
          ...this.gameState[0],
          settings: gameSettings,
        });
      },
    });
  }

  getTile(tileId: TileId) {
    return DEFAULT_DECK.get(tileId);
  }

  private handleError = () => {
    this.loadingState[1](false);
  };
}
