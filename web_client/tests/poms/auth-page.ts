import { BasePage, CommonPageOptions } from "./base-page";

const selectors = {
  password: '[data-name="password"]',
  submitButton: '[data-name="auth-submit"]',
  username: '[data-name="username"]',
};

const defaultAccount = {
  name: "test",
  password: "test",
};

export class AuthPage {
  basePage: BasePage;

  static expectedTitle = "Mahjong Web Client";

  constructor(options: CommonPageOptions) {
    this.basePage = new BasePage(options);
  }

  async fillDefault() {
    this.navigate();

    await this.basePage.page
      .locator(selectors.username)
      .fill(defaultAccount.name);
    await this.basePage.page
      .locator(selectors.password)
      .fill(defaultAccount.password);

    await this.basePage.page.locator(selectors.submitButton).click();

    await this.basePage.page.waitForFunction(
      (expectedTitle) => window.document?.title !== expectedTitle,
      AuthPage.expectedTitle
    );
  }

  navigate() {
    return this.basePage.navigate("/");
  }
}
