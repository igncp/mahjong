import { expect, test as setup } from "@playwright/test";
import { lastValueFrom } from "rxjs";

import { HttpClient } from "src/sdk/http-client";

import { API } from "./poms/api";
import { AuthPage } from "./poms/auth-page";
import { authFile } from "./utils";

// https://playwright.dev/docs/test-global-setup-teardown
// https://playwright.dev/docs/auth

setup("Login and clear test games", async ({ baseURL, page }) => {
  // Keeping this log for visibility in the tests
  console.log("debug: global.setup.ts: baseURL", baseURL);

  const api = new API(baseURL as string, page);

  const authPage = new AuthPage({
    baseURL,
    page,
  });

  await authPage.fillDefault();

  api.setUpClientURL();
  await api.setUpAuthTokenFromPage();

  const result = await lastValueFrom(HttpClient.testDeleteGames());

  expect(result.test_delete_games).toEqual(true);

  await page.context().storageState({ path: authFile });
});
