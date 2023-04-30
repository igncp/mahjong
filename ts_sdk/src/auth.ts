import { BehaviorSubject } from "rxjs";

export const tokenObserver = new BehaviorSubject<string | null>("");

export const getAuthTokenHeader = (): { Authorization?: string } => {
  const token = tokenObserver.getValue();

  return token ? ({ Authorization: `Bearer ${token}` } as const) : {};
};
