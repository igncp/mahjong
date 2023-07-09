import { gql } from "graphql-request";

import { HttpClient } from "../http-client";

export type TestDeleteGamesMutation = {
  testDeleteGames: boolean;
};

const document = gql`
  mutation {
    testDeleteGames
  }
`;

export const testDeleteGamesMutation = () => {
  return HttpClient.fetchGraphQLQuery<TestDeleteGamesMutation>(document);
};
