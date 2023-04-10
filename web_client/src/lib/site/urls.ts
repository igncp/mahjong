import { GameId, PlayerId } from "../mahjong-service";

export const SiteUrls = {
  adminGame: (gameId: GameId) => `/#/game/${gameId}/admin`,
  index: "/",
  playerGame: (gameId: GameId, userId: PlayerId) =>
    `/#/game/${gameId}/player/${userId}`,
};
