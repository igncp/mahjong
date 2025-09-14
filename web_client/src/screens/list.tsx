import type { IProps as IGameProps } from "./game";
import dynamic from "next/dynamic";
import type { FC } from "react";

export const GameScreen = dynamic(() => import("./game"), {
  ssr: false,
}) as FC<IGameProps>;
export const IndexScreen = dynamic(() => import("./index"), {
  ssr: false,
}) as FC;
export const OffscreenGame = dynamic(
  () => import("src/containers/offscreen-game"),
  {
    ssr: false,
  },
) as FC;
