import type { Page } from "@playwright/test";

export type CommonPageOptions = {
  baseURL: string | undefined;
  page: Page;
};

export const defaultBaseURL = "https://mahjong-rust.com";

export class BasePage {
  baseURL: NonNullable<CommonPageOptions["baseURL"]>;
  page: CommonPageOptions["page"];

  constructor(options: CommonPageOptions) {
    this.page = options.page;
    this.baseURL = options.baseURL || defaultBaseURL;
  }

  getRelativePath() {
    return this.page.url().replace(this.baseURL, "");
  }

  logConsole() {
    // eslint-disable-next-line no-console
    this.page.on("console", (msg) => console.log(msg.text()));
  }

  navigate(path: string) {
    return this.page.goto(`${this.baseURL}${path}`);
  }

  takeScreenshot(name: string) {
    const parsedName = name
      .replace(/[^a-z0-9]/gi, "-")
      .toLowerCase()
      .replaceAll(".png", "");

    return this.page.screenshot({
      path: ["tests/screenshots", `${parsedName}.png`].join("/"),
    });
  }
}
