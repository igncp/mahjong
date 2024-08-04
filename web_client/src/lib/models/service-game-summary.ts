import { getDeck } from "src/sdk/pkg-wrapper";

import { setDeck } from "src/sdk/service-game-summary";

export const setupServiceGameSummary = () => {
  const deck = getDeck();

  setDeck(deck);
};
