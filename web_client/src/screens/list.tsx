import dynamic from "next/dynamic";

export const GameScreen = dynamic(() => import("./game"), {
  ssr: false,
});
export const IndexScreen = dynamic(() => import("./index"), {
  ssr: false,
});
