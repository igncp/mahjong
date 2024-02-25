import i18n from "i18next";
import HttpApi from "i18next-http-backend";
import type { AppProps } from "next/app";
import qs from "qs";
import { HTML5toTouch } from "rdndmb-html5-to-touch";
import { DndProvider } from "react-dnd-multi-backend";
import { initReactI18next } from "react-i18next";
import { I18nextProvider } from "react-i18next";

import { DnDPreview } from "src/containers/dnd-preview";
import { TOKEN_KEY } from "src/lib/constants";
import { env } from "src/lib/env";
import { tokenObserver } from "src/sdk/auth";
import { setBaseUrl } from "src/sdk/http-client";
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
  i18n.on("languageChanged", (lng) => {
    localStorage.setItem("i18nextLng", lng);
  });

  if (typeof window !== "undefined") {
    const query = qs.parse(window.location.search?.replace(/^\?/, "") || "");

    if (query.token) {
      localStorage.setItem(TOKEN_KEY, query.token as string);

      window.location.replace("/");
    }

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
      <DndProvider options={HTML5toTouch}>
        {/* eslint-disable-next-line @typescript-eslint/ban-ts-comment */}
        {/* @ts-ignore React typings  */}
        <Component {...pageProps} />
        <DnDPreview />
      </DndProvider>
    </I18nextProvider>
  );
}
