// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ServiceGameSummary } from "./ServiceGameSummary";
import type { UserGetDashboardResponse } from "./UserGetDashboardResponse";

export type QueriesResponses =
  | { dashboard: UserGetDashboardResponse; type: "UserGetDashboard" }
  | { game: ServiceGameSummary; type: "UserBreakMeld" }
  | { game: ServiceGameSummary; type: "UserCreateGame" }
  | { game: ServiceGameSummary; type: "UserCreateMeld" }
  | { game: ServiceGameSummary; type: "UserDiscardTile" }
  | { game: ServiceGameSummary; type: "UserDrawTile" }
  | { game: ServiceGameSummary; type: "UserMovePlayer" };
