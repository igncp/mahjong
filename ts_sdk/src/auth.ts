import { BehaviorSubject } from "rxjs";

import { TokenClaims } from "./core";

export const tokenObserver = new BehaviorSubject<string | null>("");

export const getAuthTokenHeader = (): { Authorization?: string } => {
  const token = tokenObserver.getValue();

  return token ? ({ Authorization: `Bearer ${token}` } as const) : {};
};

export const parseJwt = (
  token: string | null,
  atob: (s: string) => string
): TokenClaims | null => {
  if (!token) {
    return null;
  }

  const [, base64Url] = token.split(".");
  const base64 = base64Url.replace(/-/g, "+").replace(/_/g, "/");

  const jsonPayload = decodeURIComponent(
    atob(base64)
      .split("")
      .map(function (c) {
        return `%${`00${c.charCodeAt(0).toString(16)}`.slice(-2)}`;
      })
      .join("")
  );

  return JSON.parse(jsonPayload);
};
