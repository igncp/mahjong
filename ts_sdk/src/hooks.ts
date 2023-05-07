import { useMemo } from "react";

import { parseJwt } from "./auth";

export const useUserTokenClaims = (
  token: string | null,
  atob: (s: string) => string
) => useMemo(() => parseJwt(token as string, atob), [token]);
