// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { RoundTileClaimed } from "./RoundTileClaimed";
import type { Wind } from "./Wind";

export type Round = {
  consecutive_same_seats: number;
  dealer_player_index: number;
  east_player_index: number;
  initial_winds: null | number;
  player_index: number;
  round_index: number;
  tile_claimed: null | RoundTileClaimed;
  wall_tile_drawn: null | number;
  wind: Wind;
};