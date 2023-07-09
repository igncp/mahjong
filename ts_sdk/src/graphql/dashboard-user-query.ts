import { gql } from "graphql-request";

import { GameId, ServicePlayer } from "../core";
import { HttpClient } from "../http-client";

export type DashboardQueryResponse = {
  playerGamesIds: GameId[];
  player: Pick<ServicePlayer, "id" | "name">;
  playerTotalScore: number;
};

const document = gql`
  {
    player {
      id
      name
    }
    playerGamesIds
    playerTotalScore
  }
`;

export const queryDashboardUserQuery = () => {
  return HttpClient.fetchGraphQLQuery<DashboardQueryResponse>(document);
};
