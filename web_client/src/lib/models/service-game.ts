import { format_tile, get_possible_melds } from "pkg";

import {
  PlayerId,
  PossibleMeld,
  ServiceGame,
  TileId,
} from "../mahjong-service";

export class ModelServiceGame {
  constructor(public data: ServiceGame) {}

  getCurrentPlayer() {
    const playerId = this.data.game.players[this.data.game.round.player_index];

    return this.data.players[playerId];
  }

  getTileString(tileId: TileId) {
    const tile = this.data.game.deck[tileId];
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
