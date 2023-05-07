import { tokenObserver } from "mahjong_sdk/src/auth";

export const getIsLoggedIn = () => {
  const token = tokenObserver.getValue();

  return !!token;
};
