import { PlaywrightTestConfig } from "@playwright/test";
import dotenv from "dotenv";
import { authFile } from "tests/utils";

dotenv.config({
  path: ".env.e2e",
});

const { E2E_BASE_URL: baseURL } = process.env;

if (!baseURL) {
  throw new Error("E2E_BASE_URL is not set");
}

const nextCheck = baseURL.includes("localhost") === false && {
  grepInvert: /@next/,
};

const config: PlaywrightTestConfig = {
  projects: [
    {
      ...nextCheck,
      name: "Setup",
      testMatch: /global.setup\.ts/,
    },
    {
      ...nextCheck,
      dependencies: ["Setup"],
      name: "Chromium",
      use: {
        storageState: authFile,
      },
    },
  ],
  use: {
    baseURL,
  },
};

export default config;
