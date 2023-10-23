import { tokenObserver } from "mahjong_sdk/dist/auth";
import { UserRole } from "mahjong_sdk/dist/core";
import { useUserTokenClaims } from "mahjong_sdk/dist/hooks";
import dynamic from "next/dynamic";
import { FC, useEffect, useState } from "react";

import type { TProps as TDashboardPlayerProps } from "./dashboard-player";

const DashboardAdmin = dynamic(() => import("./dashboard-admin"), {
  ssr: false,
}) as FC;
const DashboardPlayer = dynamic(() => import("./dashboard-player"), {
  ssr: false,
}) as FC<TDashboardPlayerProps>;

const Dashboard = () => {
  const [token, setToken] = useState(tokenObserver.getValue());
  const claims = useUserTokenClaims(token, window.atob);

  useEffect(() => {
    const subscription = tokenObserver.subscribe(setToken);

    return () => subscription.unsubscribe();
  }, []);

  if (!claims) return null;

  if (claims.role === UserRole.Admin) {
    return <DashboardAdmin />;
  }

  return <DashboardPlayer userId={claims.sub} />;
};

export default Dashboard;
