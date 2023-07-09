import { GameId, PlayerId } from "mahjong_sdk/dist/core";

export const SiteUrls = {
  adminGame: (gameId: GameId) => `/#/game/${gameId}/admin`,
  index: "/",
  playerGame: (gameId: GameId, userId: PlayerId) =>
    `/#/game/${gameId}/player/${userId}`,
};
