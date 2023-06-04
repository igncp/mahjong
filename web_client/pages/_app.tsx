import i18n from "i18next";
import HttpApi from "i18next-http-backend";
import type { AppProps } from "next/app";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";
import { initReactI18next } from "react-i18next";
import { I18nextProvider } from "react-i18next";

import { tokenObserver } from "mahjong_sdk/src/auth";
import { setBaseUrl } from "mahjong_sdk/src/http-server";
import { env } from "src/lib/env";
import "src/styles/global.css";

if (typeof window !== "undefined") {
  i18n
    .use(initReactI18next)
    .use(HttpApi)
    .init({
      backend: {
        loadPath: "/locales/{{lng}}/{{ns}}.json",
      },
      debug: process.env.NODE_ENV !== "production",
      fallbackLng: localStorage.getItem("i18nextLng") || "en",
    });
}

const setupApp = async () => {
  const TOKEN_KEY = "mahjongAuthToken";

  i18n.on("languageChanged", (lng) => {
    localStorage.setItem("i18nextLng", lng);
  });

  if (typeof window !== "undefined") {
    tokenObserver.next(localStorage.getItem(TOKEN_KEY));

    tokenObserver.subscribe((token) => {
      token
        ? localStorage.setItem(TOKEN_KEY, token)
        : localStorage.removeItem(TOKEN_KEY);
    });
  }

  setBaseUrl(env.SERVICE_URL);

  // This has an issue when building due to wasm
  const { setupServiceGameSummary } = await import(
    "src/lib/models/service-game-summary"
  ).catch(() => ({ setupServiceGameSummary: () => {} }));

  setupServiceGameSummary();
};

setupApp();

export default function App({ Component, pageProps }: AppProps) {
  return (
    <I18nextProvider i18n={i18n}>
      <DndProvider backend={HTML5Backend}>
        <Component {...pageProps} />
      </DndProvider>
    </I18nextProvider>
  );
}
