import type { AppProps } from "next/app";

import { tokenObserver } from "mahjong_sdk/src/auth";
import { setBaseUrl } from "mahjong_sdk/src/http-server";
import { env } from "src/lib/env";
import "src/styles/global.css";

const setupApp = async () => {
  const TOKEN_KEY = "mahjongAuthToken";

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
  return <Component {...pageProps} />;
}
