import dynamic from "next/dynamic";
import type { FC } from "react";
import { useEffect, useState } from "react";

import { tokenObserver } from "src/sdk/auth";
import { UserRole } from "src/sdk/core";
import { useUserTokenClaims } from "src/sdk/hooks";

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
