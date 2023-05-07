import { format_tile, get_deck, get_possible_melds_summary } from "pkg";

import {
  setDeck,
  setFormatTile,
  setGetPossibleMeldsSummary,
} from "mahjong_sdk/src/service-game-summary";

export const setupServiceGameSummary = () => {
  setFormatTile(format_tile);
  setGetPossibleMeldsSummary((game) =>
    get_possible_melds_summary(JSON.stringify(game))
  );

  const deck = get_deck();
  setDeck(deck);
};
