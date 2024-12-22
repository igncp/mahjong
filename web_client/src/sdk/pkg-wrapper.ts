import type { Deck } from "bindings/Deck";
import type { LibGetGamePlayingExtrasParam } from "bindings/LibGetGamePlayingExtrasParam";
import type { LibGetGamePlayingExtrasReturn } from "bindings/LibGetGamePlayingExtrasReturn";
import type { LibGetIsMeldParam } from "bindings/LibGetIsMeldParam";
import type { LibGetPossibleMeldsParam } from "bindings/LibGetPossibleMeldsParam";
import type { LibGetPossibleMeldsReturn } from "bindings/LibGetPossibleMeldsReturn";
import type { Tile } from "bindings/Tile";

import {
  format_tile,
  get_deck,
  get_game_playing_extras,
  get_possible_melds,
  is_chow,
  is_kong,
  is_pung,
} from "pkg";

type ObjToMap<R> = R extends Record<infer K, infer T> ? Map<K, T> : never;

export const isPung = (param: LibGetIsMeldParam) => is_pung(param);
export const isChow = (param: LibGetIsMeldParam) => is_chow(param);
export const isKong = (param: LibGetIsMeldParam) => is_kong(param);

export const formatTile = (tile: Tile) => format_tile(tile);

export const getPossibleMelds = (
  param: LibGetPossibleMeldsParam,
): LibGetPossibleMeldsReturn => get_possible_melds(param);

export const getDeck = (): Deck => get_deck();

export type PlayingExtrasParsed = Omit<
  LibGetGamePlayingExtrasReturn,
  "hand_stats" | "players_visible_melds" | "players_winds"
> & {
  hand_stats: ObjToMap<LibGetGamePlayingExtrasReturn["hand_stats"]>;
  players_visible_melds: ObjToMap<
    LibGetGamePlayingExtrasReturn["players_visible_melds"]
  >;
  players_winds: ObjToMap<LibGetGamePlayingExtrasReturn["players_winds"]>;
};

export const getGamePlayingExtras = (
  param: LibGetGamePlayingExtrasParam,
): PlayingExtrasParsed => get_game_playing_extras(param);
