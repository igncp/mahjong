import { expect, test as setup } from "@playwright/test";
import { testDeleteGamesMutation } from "mahjong_sdk/dist/graphql/test-delete-games-mutation";
import { lastValueFrom } from "rxjs";

import { API } from "./poms/api";
import { AuthPage } from "./poms/auth-page";
import { authFile } from "./utils";

// https://playwright.dev/docs/test-global-setup-teardown
// https://playwright.dev/docs/auth

setup("Login and clear test games", async ({ page, baseURL }) => {
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

  const result = await lastValueFrom(testDeleteGamesMutation());

  expect(result.testDeleteGames).toEqual(true);

  await page.context().storageState({ path: authFile });
});
