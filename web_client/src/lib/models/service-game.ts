import type { PossibleMeld } from "bindings/PossibleMeld";
import type { ServiceGame } from "bindings/ServiceGame";

import { format_tile, get_possible_melds } from "pkg";

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

  getPossibleMelds(): PossibleMeld[] {
    const possibleMelds = get_possible_melds(JSON.stringify(this.data));

    return possibleMelds;
  }

  getTileString(tileId: TileId) {
    const tile = getDeck()[tileId];
    const tileString = format_tile(tile);

    return `[${tileString}]`;
  }
}
