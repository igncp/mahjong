import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Alert } from "react-native";
import { BehaviorSubject, pairwise } from "rxjs";

export enum NetState {
  Disconnected,
  Connected,
}

export const netState$ = new BehaviorSubject<NetState>(NetState.Connected);

export const useIsConnected = () => {
  const { t } = useTranslation();
  const [isConnected, setIsConnected] = useState(true);

  useEffect(() => {
    const netSubs = netState$
      .pipe(pairwise())
      .subscribe(([prev, newStatus]) => {
        if (prev !== newStatus) {
          setIsConnected(newStatus === NetState.Connected);
        }

        if (
          newStatus === NetState.Disconnected &&
          prev === NetState.Connected
        ) {
          Alert.alert(
            t("alert.disconnected", "You are currently disconnected")
          );
        }
      });

    return () => {
      netSubs.unsubscribe();
    };
  }, [t]);

  return isConnected;
};
