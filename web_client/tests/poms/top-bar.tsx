import { BasePage, CommonPageOptions } from "./base-page";

const selectors = {
  signout: '[data-name="signout-button"]',
};

export class TopBar {
  basePage: BasePage;

  constructor(options: CommonPageOptions) {
    this.basePage = new BasePage(options);
  }

  async signout() {
    const locator = this.basePage.page.locator(selectors.signout);

    await locator.click();
  }
}
