import AsyncStorage from "@react-native-async-storage/async-storage";
import { NavigationContainer } from "@react-navigation/native";
import i18n from "i18next";
import React, { useEffect, useState } from "react";
import { I18nextProvider } from "react-i18next";
import {
  catchError,
  first,
  from,
  fromEvent,
  map,
  mergeMap,
  of,
  tap,
} from "rxjs";

import Router from "./Router";
import { I18N_KEY, InternalLocale, initI18n } from "./lib/i18n";
import { setTimeLocale } from "./lib/time";

const useSetupI18n = <A,>(setI18nInstance: (val: A) => void) => {
  useEffect(() => {
    const subscription = from(AsyncStorage.getItem(I18N_KEY))
      .pipe(
        first(),
        mergeMap((language) => from(initI18n(language))),
        mergeMap(() => {
          setI18nInstance(i18n as A);
          setTimeLocale((i18n.language as InternalLocale) || "en");

          return fromEvent(i18n, "languageChanged").pipe(
            mergeMap((lng) => {
              setTimeLocale(lng);
              return from(AsyncStorage.setItem(I18N_KEY, lng)).pipe(
                map(() => lng)
              );
            }),
            tap((lng) => {
              console.log("Language saved correctly to:", lng);
            }),
            catchError((error) => {
              console.log("Error while changing language", error);

              return of();
            })
          );
        })
      )
      .subscribe();

    return () => {
      subscription.unsubscribe();
    };
  }, [setI18nInstance]);
};

const App = () => {
  const [i18nInstance, setI18nInstance] = useState<null | typeof i18n>(null);

  useSetupI18n(setI18nInstance);

  if (!i18nInstance) {
    // @TODO: Display a loading screen
    return null;
  }

  return (
    <I18nextProvider i18n={i18nInstance}>
      <NavigationContainer>
        <Router />
      </NavigationContainer>
    </I18nextProvider>
  );
};

export default App;
