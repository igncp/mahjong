import { Subject } from "rxjs";

import {
  Deck,
  GameSettingsSummary,
  PossibleMeld,
  ServiceGameSummary,
  Tile,
  TileId,
} from "./core";
import { HttpClient } from "./http-client";

export type ModelState<A> = [A, (v: A) => void];

let deck: Deck;
let format_tile: (tile: Tile) => string;
let get_possible_melds_summary: (game: ServiceGameSummary) => PossibleMeld[];

export const setDeck = (newDeck: Deck) => {
  deck = newDeck;
};
export const getDeck = () => deck;

export const setFormatTile = (newFormatTile: typeof format_tile) => {
  format_tile = newFormatTile;
};

export const getTile = (tileId: TileId) => {
  return deck.get(tileId) as Tile;
};

export const setGetPossibleMeldsSummary = (
  newGetPossibleMeldsSummary: typeof get_possible_melds_summary
) => {
  get_possible_melds_summary = newGetPossibleMeldsSummary;
};

export enum ModelServiceGameSummaryError {
  INVALID_SAY_MAHJONG = "INVALID_SAY_MAHJONG",
}

export class ModelServiceGameSummary {
  public isLoading = false;

  public gameState!: ModelState<ServiceGameSummary>;
  public loadingState!: ModelState<boolean>;
  public errorEmitter$ = new Subject<ModelServiceGameSummaryError>();

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

  getPlayerHandWithoutMelds() {
    const { hand } = this.gameState[0].game_summary;

    return hand.filter((tile) => !tile.set_id);
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
      const tile = this.getTile(tileId);
      const tileString = format_tile(tile);

      return `[${tileString}]`;
    } catch (err) {
      console.log("debug: service-game-summary.ts: err", err);
    }
    return "";
  }

  getPossibleMelds(): PossibleMeld[] {
    try {
      const possibleMelds = get_possible_melds_summary(this.gameState[0]);

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
      error: (error) => {
        console.log("debug: service-game-summary.ts: error", error);
        this.handleError(ModelServiceGameSummaryError.INVALID_SAY_MAHJONG);
      },
      next: (newGame) => {
        this.loadingState[1](false);
        this.gameState[1](newGame);
      },
    });
  }

  sortHands(tiles?: TileId[]) {
    if (this.loadingState[0]) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userSortHand(this.gameState[0].game_summary.id, {
      game_version: this.gameState[0].game_summary.version,
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

    if (tiles) {
      const tileIdToIndex = new Map<TileId, number>();
      tiles?.forEach((tileId, index) => {
        tileIdToIndex.set(tileId, index);
      });

      const newHand = this.gameState[0].game_summary.hand
        .slice()
        .sort((a, b) => {
          const aIndex = tileIdToIndex.get(a.id);
          const bIndex = tileIdToIndex.get(b.id);

          if (aIndex === undefined || bIndex === undefined) {
            return 0;
          }

          return aIndex - bIndex;
        });

      this.gameState[1]({
        ...this.gameState[0],
        game_summary: {
          ...this.gameState[0].game_summary,
          hand: newHand,
        },
      });
    }
  }

  setGameSettings(gameSettings: GameSettingsSummary) {
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
    return getTile(tileId);
  }

  private handleError = (error?: ModelServiceGameSummaryError) => {
    if (error) {
      this.errorEmitter$.next(error);
    }

    this.loadingState[1](false);
  };
}
