import dynamic from "next/dynamic";
import { useEffect, useMemo, useState } from "react";

import { tokenObserver } from "mahjong_sdk/src/auth";
import { UserRole } from "mahjong_sdk/src/core";
import { parseJwt } from "src/lib/auth";

const DashboardAdmin = dynamic(() => import("./dashboard-admin"), {
  ssr: false,
});
const DashboardPlayer = dynamic(() => import("./dashboard-player"), {
  ssr: false,
});

const Dashboard = () => {
  const [token, setToken] = useState(tokenObserver.getValue());

  useEffect(() => {
    const subscription = tokenObserver.subscribe(setToken);

    return () => subscription.unsubscribe();
  }, []);

  const claims = useMemo(() => parseJwt(token as string), [token]);

  if (claims.role === UserRole.Admin) {
    return <DashboardAdmin />;
  }

  return <DashboardPlayer userId={claims.sub} />;
};

export default Dashboard;
