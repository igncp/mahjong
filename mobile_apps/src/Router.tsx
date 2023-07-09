import { createNativeStackNavigator } from "@react-navigation/native-stack";
import { tokenObserver } from "mahjong_sdk/dist/auth";
import React, { useEffect, useState } from "react";

import { setupApp } from "./lib/setup";
import { AuthScreen } from "./screens/auth";
import { DashboardScreen } from "./screens/dashboard";
import { GameScreen } from "./screens/game";

const Stack = createNativeStackNavigator();

const Router = () => {
  const [isLoggedIn, setIsLoggedIn] = useState<boolean | null>(null);

  useEffect(() => {
    setupApp();

    const subscription = tokenObserver.subscribe({
      next: (newToken) => {
        setIsLoggedIn(!!newToken);
      },
    });

    return subscription.unsubscribe;
  }, []);

  return (
    <Stack.Navigator>
      {isLoggedIn ? (
        <>
          <Stack.Screen component={DashboardScreen} name="Home" />
          {/* eslint-disable-next-line @typescript-eslint/no-explicit-any */}
          <Stack.Screen component={GameScreen as any} name="Game" />
        </>
      ) : (
        <Stack.Screen component={AuthScreen} name="Home" />
      )}
    </Stack.Navigator>
  );
};

export default Router;
