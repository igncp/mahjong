import dynamic from "next/dynamic";
import { useEffect, useState } from "react";

import { tokenObserver } from "mahjong_sdk/src/auth";
import { UserRole } from "mahjong_sdk/src/core";
import { useUserTokenClaims } from "mahjong_sdk/src/hooks";

const DashboardAdmin = dynamic(() => import("./dashboard-admin"), {
  ssr: false,
});
const DashboardPlayer = dynamic(() => import("./dashboard-player"), {
  ssr: false,
});

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
