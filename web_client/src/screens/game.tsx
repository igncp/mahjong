import dynamic from "next/dynamic";
import { FC } from "react";

import type { IProps as IGameAdminProps } from "src/containers/game/admin";
import type { IProps as IGamePlayerProps } from "src/containers/game/player";
import { GameId, PlayerId } from "src/sdk/core";

const GameAdmin = dynamic(() => import("src/containers/game/admin"), {
  ssr: false,
}) as FC<IGameAdminProps>;
const GamePlayer = dynamic(() => import("src/containers/game/player"), {
  ssr: false,
}) as FC<IGamePlayerProps>;

export interface IProps {
  gameId: GameId;
  gameType: string;
  userId?: PlayerId;
}

const Game = ({ gameId, gameType, userId }: IProps) => {
  switch (true) {
    case gameType === "admin":
      return <GameAdmin gameId={gameId} />;
    case gameType === "player" && !!userId:
      return <GamePlayer gameId={gameId} userId={userId as string} />;
    default:
      return null;
  }
};

export default Game;
