import { tokenObserver } from "mahjong_sdk/dist/auth";
import qs from "query-string";

import { env } from "./env";

export const getIsLoggedIn = () => {
  const token = tokenObserver.getValue();

  return !!token;
};

export const githubAuth = {
  login: () => {
    const params = qs.stringify({
      client_id: env.GITHUB_CLIENT_ID,
      redirect_uri:
        env.GITHUB_REDIRECT ||
        "https://mahjong-rust.com/api/v1/github_callback",
    });

    window.location.href = `https://github.com/login/oauth/authorize?${params}`;
  },
};
