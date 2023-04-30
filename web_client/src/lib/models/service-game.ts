import { format_tile, get_possible_melds } from "pkg";

import {
  PlayerId,
  PossibleMeld,
  ServiceGame,
  TileId,
} from "mahjong_sdk/src/core";

import { DEFAULT_DECK } from "./service-game-summary";

export class ModelServiceGame {
  constructor(public data: ServiceGame) {}

  getCurrentPlayer() {
    const playerId = this.data.game.players[this.data.game.round.player_index];

    return this.data.players[playerId];
  }

  getTileString(tileId: TileId) {
    const tile = DEFAULT_DECK[tileId];
    const tileString = format_tile(tile);

    return `[${tileString}]`;
  }

  getPlayerScore(playerId: PlayerId) {
    return this.data.game.score[playerId];
  }

  getPossibleMelds(): PossibleMeld[] {
    const possibleMelds = get_possible_melds(JSON.stringify(this.data));

    return possibleMelds;
  }
}
