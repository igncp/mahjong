import type { GameId, PlayerId } from "src/sdk/core";

export const SiteUrls = {
  adminGame: (gameId: GameId) => `/#/game/${gameId}/admin`,
  index: "/",
  offscreenGame: "/#/offscreen-game",
  playerGame: (gameId: GameId, userId: PlayerId) =>
    `/#/game/${gameId}/player/${userId}`,
};
