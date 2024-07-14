import dynamic from "next/dynamic";
import type { FC } from "react";

import type { IProps as IGameProps } from "./game";

export const GameScreen = dynamic(() => import("./game"), {
  ssr: false,
}) as FC<IGameProps>;
export const IndexScreen = dynamic(() => import("./index"), {
  ssr: false,
}) as FC;
