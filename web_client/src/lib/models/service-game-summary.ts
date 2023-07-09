import {
  setDeck,
  setFormatTile,
  setGetPossibleMeldsSummary,
} from "mahjong_sdk/dist/service-game-summary";

import { format_tile, get_deck, get_possible_melds_summary } from "pkg";

export const setupServiceGameSummary = () => {
  setFormatTile(format_tile);
  setGetPossibleMeldsSummary((game) =>
    get_possible_melds_summary(JSON.stringify(game))
  );

  const deck = get_deck();
  setDeck(deck);
};
