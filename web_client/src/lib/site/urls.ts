import { GameId, PlayerId } from "../mahjong-service";

export const SiteUrls = {
  adminGame: (gameId: GameId) => `/#/game/${gameId}/admin`,
  dashboardAdmin: "/#/dashboard/admin",
  dashboardPlayer: (userId: PlayerId) => `/#/dashboard/player/${userId}`,
  index: "/",
  playerGame: (gameId: GameId, userId: PlayerId) =>
    `/#/game/${gameId}/player/${userId}`,
};
