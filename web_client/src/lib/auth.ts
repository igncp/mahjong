import qs from "query-string";

import { tokenObserver } from "src/sdk/auth";
import { HttpClient } from "src/sdk/http-client";

import { env } from "./env";
import { uuid } from "./utils";

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

export const anonymousAuth = {
  login: () => {
    const token = uuid();

    return HttpClient.setAuthAnonymous({
      id_token: token,
    });
  },
};
