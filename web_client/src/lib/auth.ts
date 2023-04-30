import { tokenObserver } from "mahjong_sdk/src/auth";
import { TokenClaims } from "mahjong_sdk/src/core";

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
