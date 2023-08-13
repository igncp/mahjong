import { API_URL } from "@env";
import AsyncStorage from "@react-native-async-storage/async-storage";
import NetInfo from "@react-native-community/netinfo";
import { tokenObserver } from "mahjong_sdk/dist/auth";
import { Deck, Tile } from "mahjong_sdk/dist/core";
import { HttpClient, setBaseUrl } from "mahjong_sdk/dist/http-client";
import { setDeck } from "mahjong_sdk/dist/service-game-summary";
import { first, from, tap } from "rxjs";

import { NetState, netState$ } from "./net";

const TOKEN_KEY = "mahjong_rust_token";
const defaultAPI = "https://mahjong-rust.com";

export const setupApp = () => {
  setBaseUrl(API_URL || defaultAPI);

  const tokenSubs = from(AsyncStorage.getItem(TOKEN_KEY))
    .pipe(
      first(),
      tap((token) => {
        if (token) {
          tokenObserver.next(token);
        }

        tokenObserver.subscribe((newToken) => {
          if (newToken) {
            AsyncStorage.setItem(TOKEN_KEY, newToken);
          } else {
            AsyncStorage.removeItem(TOKEN_KEY);
          }
        });
      })
    )
    .subscribe();

  const unsubscribeNetInfo = NetInfo.addEventListener((state) => {
    netState$.next(
      state.isConnected ? NetState.Connected : NetState.Disconnected
    );
  });

  HttpClient.getDeck()
    .pipe(first())
    .subscribe((deck) => {
      const entries = Object.entries(deck).map((entry) => [
        Number(entry[0]),
        entry[1],
      ]) as [number, Tile][];
      const deckMap = new Map(entries) as Deck;
      setDeck(deckMap);
    });

  return () => {
    unsubscribeNetInfo();
    tokenSubs.unsubscribe();
  };
};
