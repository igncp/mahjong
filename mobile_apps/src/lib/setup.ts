import { API_URL } from "@env";
import AsyncStorage from "@react-native-async-storage/async-storage";
import { tokenObserver } from "mahjong_sdk/dist/auth";
import { Deck, Tile } from "mahjong_sdk/dist/core";
import { HttpClient, setBaseUrl } from "mahjong_sdk/dist/http-client";
import { setDeck } from "mahjong_sdk/dist/service-game-summary";
import { first, from, tap } from "rxjs";

const TOKEN_KEY = "mahjong_rust_token";

export const setupApp = () => {
  setBaseUrl(API_URL);

  from(AsyncStorage.getItem(TOKEN_KEY))
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
};
