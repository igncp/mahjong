import { Page } from "@playwright/test";

export type CommonPageOptions = {
  page: Page;
  baseURL: string | undefined;
};

export const defaultBaseURL = "https://mahjong-rust.com";

export class BasePage {
  page: CommonPageOptions["page"];
  baseURL: NonNullable<CommonPageOptions["baseURL"]>;

  constructor(options: CommonPageOptions) {
    this.page = options.page;
    this.baseURL = options.baseURL || defaultBaseURL;
  }

  navigate(path: string) {
    return this.page.goto(`${this.baseURL}${path}`);
  }

  getRelativePath() {
    return this.page.url().replace(this.baseURL, "");
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
