import { tokenObserver } from "mahjong_sdk/dist/auth";

export const TOKEN_KEY = "mahjongAuthToken";

export const getIsLoggedIn = () => {
  const token = tokenObserver.getValue();

  return !!token;
};
