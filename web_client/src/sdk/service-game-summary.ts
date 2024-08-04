import type { Deck } from "bindings/Deck";
import type { GameSettingsSummary } from "bindings/GameSettingsSummary";
import type { Hand } from "bindings/Hand";
import type { PossibleMeld } from "bindings/PossibleMeld";
import type { ServiceGameSummary } from "bindings/ServiceGameSummary";
import type { Tile } from "bindings/Tile";
import { Subject } from "rxjs";

import {
  formatTile,
  getGamePlayingExtras,
  isChow,
  isKong,
  isPung,
} from "./pkg-wrapper";

import type { TileId } from "./core";
import { HttpClient } from "./http-client";

export type ModelState<A> = [A | null, (v: A) => void];

let deck: Deck;

export const setDeck = (newDeck: Deck) => {
  deck = newDeck;
};

export const getDeck = () => deck;

export const getTile = (tileId: TileId) => deck[tileId] as Tile;

export enum ModelServiceGameSummaryError {
  INVALID_SAY_MAHJONG = "INVALID_SAY_MAHJONG",
}

export class ModelServiceGameSummary {
  errorEmitter$ = new Subject<ModelServiceGameSummaryError>();

  gameState!: ModelState<ServiceGameSummary>;
  private handleError = (error?: ModelServiceGameSummaryError) => {
    if (error) {
      this.errorEmitter$.next(error);
    }

    this.loadingState[1](false);
  };
  isLoading = false;

  loadingState!: ModelState<boolean>;

  breakMeld(setId: string) {
    const [gameState] = this.gameState;

    if (this.loadingState[0] || !gameState) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userBreakMeld(gameState.game_summary.id, {
      player_id: gameState.game_summary.player_id,
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

  claimTile() {
    const [gameState] = this.gameState;

    if (this.loadingState[0] || !gameState) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userClaimTile(gameState.game_summary.id, {
      player_id: gameState.game_summary.player_id,
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

  createMeld(meld: PossibleMeld) {
    const [gameState] = this.gameState;

    if (this.loadingState[0] || !gameState) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userCreateMeld(gameState.game_summary.id, {
      is_concealed: meld.is_concealed,
      is_upgrade: meld.is_upgrade,
      player_id: gameState.game_summary.player_id,
      tiles: meld.tiles,
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
    const [gameState] = this.gameState;

    if (this.loadingState[0] || !gameState) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userDiscardTile(gameState.game_summary.id, {
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

  getGamePlayingExtras() {
    const [gameState] = this.gameState;

    if (!gameState) return null;

    return getGamePlayingExtras(gameState);
  }

  getIsChow(tiles: TileId[]) {
    return isChow(tiles);
  }

  getIsKong(tiles: TileId[]) {
    return isKong(tiles);
  }

  getIsPung(tiles: TileId[]) {
    return isPung(tiles);
  }

  getPlayerHandWithoutMelds(): Hand | null {
    const [gameState] = this.gameState;

    if (!gameState) return null;

    const { hand } = gameState.game_summary;

    if (!hand?.list) return null;

    return { ...hand, list: hand.list.filter((tile) => !tile.set_id) };
  }

  getPlayingPlayerIndex() {
    const [gameState] = this.gameState;

    if (!gameState) return null;

    return gameState.game_summary.players.findIndex(
      (player) => player === gameState.game_summary.player_id,
    );
  }

  getShareLink(gameId: string) {
    return `${window.location.origin}/#/game/${gameId}/join`;
  }

  getTile(tileId: TileId) {
    return getTile(tileId);
  }

  getTileString(tileId: TileId) {
    try {
      const tile = this.getTile(tileId);
      const tileString = formatTile(tile);

      return `[${tileString}]`;
    } catch (err) {
      console.error("debug: service-game-summary.ts: err", err);
    }

    return "";
  }

  passRound() {
    const [gameState] = this.gameState;

    if (this.loadingState[0] || !gameState) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userPassRound(gameState.game_summary.id, {
      player_id: gameState.game_summary.player_id,
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

  sayMahjong() {
    const [gameState] = this.gameState;

    if (this.loadingState[0] || !gameState) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userSayMahjong(gameState.game_summary.id, {
      player_id: gameState.game_summary.player_id,
    }).subscribe({
      error: (error) => {
        console.error("debug: service-game-summary.ts: error", error);
        this.handleError(ModelServiceGameSummaryError.INVALID_SAY_MAHJONG);
      },
      next: (newGame) => {
        this.loadingState[1](false);
        this.gameState[1](newGame);
      },
    });
  }

  setGameSettings(gameSettings: GameSettingsSummary) {
    const [gameState] = this.gameState;

    if (this.loadingState[0] || !gameState) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userSetGameSettings(gameState.game_summary.id, {
      player_id: gameState.game_summary.player_id,
      settings: gameSettings,
    }).subscribe({
      next: () => {
        this.loadingState[1](false);

        const [gameState2] = this.gameState;

        if (!gameState2) return;

        this.gameState[1]({
          ...gameState2,
          settings: gameSettings,
        });
      },
    });
  }

  sortHands(tiles?: TileId[]) {
    const [gameState] = this.gameState;

    if (this.loadingState[0] || !gameState) {
      return;
    }

    this.loadingState[1](true);

    HttpClient.userSortHand(gameState.game_summary.id, {
      game_version: gameState.game_summary.version,
      player_id: gameState.game_summary.player_id,
      tiles: tiles || null,
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

      const prevHand = gameState.game_summary.hand;

      if (!prevHand) return;

      const newHand = { ...prevHand };

      newHand.list = newHand.list.slice().sort((a, b) => {
        const aIndex = tileIdToIndex.get(a.id);
        const bIndex = tileIdToIndex.get(b.id);

        if (aIndex === undefined || bIndex === undefined) {
          return 0;
        }

        return aIndex - bIndex;
      });

      this.gameState[1]({
        ...gameState,
        game_summary: {
          ...gameState.game_summary,
          hand: newHand,
        },
      });
    }
  }

  updateStates(
    gameState: ModelState<ServiceGameSummary>,
    loadingState: ModelState<boolean>,
  ) {
    this.gameState = gameState;
    this.loadingState = loadingState;
  }
}
