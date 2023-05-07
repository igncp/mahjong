import { decode as atob } from "base-64";
import { useObservable } from "rxjs-hooks";

import { tokenObserver } from "mahjong_sdk/src/auth";
import { useUserTokenClaims } from "mahjong_sdk/src/hooks";

export const useUserId = () => {
  const token = useObservable(() => tokenObserver);

  return useUserTokenClaims(token, atob)?.sub;
};
