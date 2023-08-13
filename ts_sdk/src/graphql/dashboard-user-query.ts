import { gql } from "graphql-request";

import { ServicePlayer, ServicePlayerGame } from "../core";
import { HttpClient } from "../http-client";

export type DashboardQueryResponse = {
  playerGames: Pick<ServicePlayerGame, "id" | "createdAt" | "updatedAt">[];
  player: Pick<ServicePlayer, "id" | "name">;
  playerTotalScore: number;
};

const document = gql`
  {
    player {
      id
      name
    }
    playerGames {
      createdAt
      id
      updatedAt
    }
    playerTotalScore
  }
`;

export const queryDashboardUserQuery = () => {
  return HttpClient.fetchGraphQLQuery<DashboardQueryResponse>(document);
};
