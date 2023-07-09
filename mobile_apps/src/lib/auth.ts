import { decode as atob } from "base-64";
import { tokenObserver } from "mahjong_sdk/dist/auth";
import { useUserTokenClaims } from "mahjong_sdk/dist/hooks";
import { useObservable } from "rxjs-hooks";

export const useUserId = () => {
  const token = useObservable(() => tokenObserver);

  return useUserTokenClaims(token, atob)?.sub;
};
