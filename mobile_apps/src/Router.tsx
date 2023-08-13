import { createNativeStackNavigator } from "@react-navigation/native-stack";
import { tokenObserver } from "mahjong_sdk/dist/auth";
import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { setupApp } from "./lib/setup";
import { AuthScreen } from "./screens/auth";
import { DashboardScreen } from "./screens/dashboard";
import { GameScreen } from "./screens/game";

const Stack = createNativeStackNavigator();

const Router = () => {
  const [isLoggedIn, setIsLoggedIn] = useState<boolean | null>(null);
  const { t } = useTranslation();

  useEffect(() => {
    const unsubscribe = setupApp();

    const tokenSubscription = tokenObserver.subscribe({
      next: (newToken) => {
        setIsLoggedIn(!!newToken);
      },
    });

    return () => {
      unsubscribe();
      tokenSubscription.unsubscribe();
    };
  }, []);

  return (
    <Stack.Navigator>
      {isLoggedIn ? (
        <>
          <Stack.Screen component={DashboardScreen} name={t("router.home")} />
          <Stack.Screen
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            component={GameScreen as any}
            name={t("router.game")}
          />
        </>
      ) : (
        <Stack.Screen
          component={AuthScreen}
          name="Home"
          options={{ headerShown: false }}
        />
      )}
    </Stack.Navigator>
  );
};

export default Router;
