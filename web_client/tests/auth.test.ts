import { expect, test } from "@playwright/test";

import { AuthPage } from "./poms/auth-page";
import { DashboardPage } from "./poms/dashboard-page";
import { TopBar } from "./poms/top-bar";

test.beforeEach(async ({ baseURL, page }) => {
  const dashboardPage = new DashboardPage({ baseURL, page });
  const topBar = new TopBar({ baseURL, page });

  await dashboardPage.navigate();
  await topBar.signout();
});

test("Page title", async ({ baseURL, page }) => {
  const authPage = new AuthPage({ baseURL, page });

  await authPage.navigate();

  expect(await page.title()).toEqual(AuthPage.expectedTitle);
});

test("After signup/login it lands on the dashboard", async ({
  page,
  baseURL,
}) => {
  const authPage = new AuthPage({ baseURL, page });
  const dashboardPage = new DashboardPage({ baseURL, page });

  await authPage.fillDefault();

  expect(...(await dashboardPage.isOnPage())).toEqual(true);
});
