import { expect, test } from "@playwright/test";

import { DashboardPage } from "./poms/dashboard-page";

test.beforeEach(async ({ baseURL, page }) => {
  const dashboardPage = new DashboardPage({ baseURL, page });
  await dashboardPage.navigate();
});

test("Page title", async ({ page }) => {
  expect(await page.title()).toEqual(DashboardPage.expectedTitle);
});

test("When creating a game, it is listed", async ({ page, baseURL }) => {
  const dashboardPage = new DashboardPage({ baseURL, page });

  expect(await dashboardPage.getGamesCount()).toEqual(0);

  await dashboardPage.clickCreateGame();
  await dashboardPage.navigate();

  expect(await dashboardPage.getGamesCount()).toEqual(1);
});

test("Can change the display name", async ({ page, baseURL }) => {
  const dashboardPage = new DashboardPage({ baseURL, page });
  const newName = await dashboardPage.getNewDisplayName();

  expect(await dashboardPage.getDisplayName()).not.toEqual(newName);

  await dashboardPage.editDisplayName(newName);

  await page.reload();

  expect(await dashboardPage.getDisplayName()).toEqual(newName);
});
