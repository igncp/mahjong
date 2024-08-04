import type { ServiceGame } from "bindings/ServiceGame";

import { formatTile, getPossibleMelds } from "src/sdk/pkg-wrapper";

import type { PlayerId, TileId } from "src/sdk/core";
import { getDeck } from "src/sdk/service-game-summary";

export class ModelServiceGame {
  constructor(public data: ServiceGame) {}

  getCurrentPlayer() {
    const playerId = this.data.game.players[this.data.game.round.player_index];

    return this.data.players[playerId];
  }

  getPlayerScore(playerId: PlayerId) {
    return this.data.game.score[playerId];
  }

  getPossibleMelds() {
    return getPossibleMelds(this.data);
  }

  getTileString(tileId: TileId) {
    const tile = getDeck()[tileId];
    const tileString = formatTile(tile);

    return `[${tileString}]`;
  }
}
