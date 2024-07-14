import { AuthPage } from "./auth-page";
import type { CommonPageOptions } from "./base-page";
import { BasePage } from "./base-page";

const selectors = {
  createGameButton: '[data-name="create-game-button"]',
  displayNameButton: '[data-name="display-name-button"]',
  displayNameContent: '[data-name="display-name-content"]',
  displayNameInput: '[data-name="display-name-input"]',
  displayNameTrigger: '[data-name="display-name"]',
  gameItem: '[data-name="existing-game"]',
};

export class DashboardPage {
  basePage: BasePage;
  static expectedPath = "/";

  static expectedTitle = "Mahjong Dashboard";

  constructor(options: CommonPageOptions) {
    this.basePage = new BasePage(options);
  }

  async clickCreateGame(): Promise<void> {
    const button = this.basePage.page.locator(selectors.createGameButton);

    await button.click();

    await this.waitForTitle(false);
  }

  async editDisplayName(newName: string) {
    const displayName = this.basePage.page.locator(
      selectors.displayNameTrigger
    );

    await displayName.click();

    await this.basePage.page.locator(selectors.displayNameInput).fill(newName);

    await this.basePage.page.locator(selectors.displayNameButton).click();

    await this.basePage.page.waitForSelector(selectors.displayNameContent);
  }

  getDisplayName() {
    return this.basePage.page.locator(selectors.displayNameContent).innerText();
  }

  async getGamesCount(): Promise<number> {
    const items = this.basePage.page.locator(selectors.gameItem);

    return await items.count();
  }

  async getNewDisplayName() {
    const existingName = await this.getDisplayName();

    return existingName.includes("Variant 1")
      ? "Test Name Variant 2"
      : "Test Name Variant 1";
  }

  async isOnPage(): Promise<[boolean, string]> {
    // If the title is the same, this check is not valid
    if (AuthPage.expectedTitle === DashboardPage.expectedTitle) {
      return [false, "The titles are the same"];
    }

    const pageTitle = await this.basePage.page.title();
    const pagePath = await this.basePage.getRelativePath();

    if (pagePath !== DashboardPage.expectedPath) {
      return [false, `The path is not correct: ${pagePath}`];
    }

    return [
      pageTitle === DashboardPage.expectedTitle,
      `The titles are not the same: "${pageTitle}" !== "${DashboardPage.expectedTitle}"`,
    ];
  }

  async navigate() {
    await this.basePage.navigate("/");
    await this.waitForTitle();
  }

  private async waitForTitle(isEqual = true) {
    await this.basePage.page.waitForFunction(
      ([expectedTitle, isEqualVal]) =>
        (document.title === expectedTitle) === isEqualVal,
      [DashboardPage.expectedTitle, isEqual]
    );
  }
}
