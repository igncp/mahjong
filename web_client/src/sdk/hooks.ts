import {
  DependencyList,
  EffectCallback,
  useEffect,
  useMemo,
  useRef,
} from "react";

import { parseJwt } from "./auth";

export const useUserTokenClaims = (
  token: null | string,
  atob: (s: string) => string
) => useMemo(() => parseJwt(token as string, atob), [token]);

export const useEffectExceptOnMount = (
  effect: EffectCallback,
  dependencies: DependencyList
) => {
  const mounted = useRef(false);

  useEffect(() => {
    if (mounted.current) {
      effect();
    } else {
      mounted.current = true;
    }
  }, dependencies);
};
