import { Page } from "@playwright/test";
import { tokenObserver } from "mahjong_sdk/dist/auth";
import { setBaseUrl } from "mahjong_sdk/dist/http-client";

import { TOKEN_KEY } from "src/lib/auth";

export class API {
  constructor(private pageBaseURL: string, private page: Page) {}

  async setUpAuthTokenFromPage() {
    const token = await this.page.evaluate(() => {
      const token = localStorage.getItem("mahjongAuthToken");

      return token;
    }, [TOKEN_KEY]);

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
