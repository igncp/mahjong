import dynamic from "next/dynamic";

export const DashboardAdmin = dynamic(() => import("./dashboard-admin"), {
  ssr: false,
});
export const DashboardPlayer = dynamic(() => import("./dashboard-player"), {
  ssr: false,
});
export const GameScreen = dynamic(() => import("./game"), {
  ssr: false,
});
export const IndexScreen = dynamic(() => import("./index"), {
  ssr: false,
});
