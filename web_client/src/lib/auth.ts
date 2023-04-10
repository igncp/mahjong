import { BehaviorSubject } from "rxjs";

import { TokenClaims } from "./mahjong-service";

const TOKEN_KEY = "mahjongAuthToken";

export const tokenObserver = new BehaviorSubject<string | null>(
  typeof window !== "undefined" ? localStorage.getItem(TOKEN_KEY) : null
);

tokenObserver.subscribe((token) => {
  token
    ? localStorage.setItem(TOKEN_KEY, token)
    : localStorage.removeItem(TOKEN_KEY);
});

export const getAuthTokenHeader = (): { Authorization?: string } => {
  const token = tokenObserver.getValue();

  return token ? ({ Authorization: `Bearer ${token}` } as const) : {};
};

export const getIsLoggedIn = () => {
  const token = tokenObserver.getValue();

  return !!token;
};

export const parseJwt = (token: string): TokenClaims => {
  const [, base64Url] = token.split(".");
  const base64 = base64Url.replace(/-/g, "+").replace(/_/g, "/");

  const jsonPayload = decodeURIComponent(
    window
      .atob(base64)
      .split("")
      .map(function (c) {
        return `%${`00${c.charCodeAt(0).toString(16)}`.slice(-2)}`;
      })
      .join("")
  );

  return JSON.parse(jsonPayload);
};
