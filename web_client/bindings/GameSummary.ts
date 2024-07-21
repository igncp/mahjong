// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Board } from "./Board";
import type { BonusTiles } from "./BonusTiles";
import type { GamePhase } from "./GamePhase";
import type { GameStyle } from "./GameStyle";
import type { Hand } from "./Hand";
import type { OtherPlayerHands } from "./OtherPlayerHands";
import type { Players } from "./Players";
import type { RoundSummary } from "./RoundSummary";
import type { Score } from "./Score";

export type GameSummary = {
  board: Board;
  bonus_tiles: BonusTiles;
  draw_wall_count: number;
  hand: Hand | null;
  id: string;
  other_hands: OtherPlayerHands;
  phase: GamePhase;
  player_id: string;
  players: Players;
  round: RoundSummary;
  score: Score;
  style: GameStyle;
  version: string;
};