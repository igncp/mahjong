import { Page } from "@playwright/test";

import { TOKEN_KEY } from "src/lib/constants";
import { tokenObserver } from "src/sdk/auth";
import { setBaseUrl } from "src/sdk/http-client";

export class API {
  constructor(private pageBaseURL: string, private page: Page) {}

  async setUpAuthTokenFromPage() {
    const token = await this.page.evaluate(
      ([tokenKey]) => localStorage.getItem(tokenKey),
      [TOKEN_KEY]
    );

    if (token) {
      tokenObserver.next(token);
    }
  }

  setUpClientURL() {
    const url = (() => {
      if (this.pageBaseURL.includes("localhost")) {
        return "http://0.0.0.0:3000";
      }

      if (this.pageBaseURL.includes("mahjong-rust.com")) {
        return "https://mahjong-rust.com/api";
      }

      throw new Error(`Unknown pageBaseURL: ${this.pageBaseURL}`);
    })();

    setBaseUrl(url);
  }
}
