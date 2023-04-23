import dynamic from "next/dynamic";

import { GameId, PlayerId } from "src/lib/mahjong-service";

const GameAdmin = dynamic(() => import("src/containers/game/admin"), {
  ssr: false,
});
const GamePlayer = dynamic(() => import("src/containers/game/player"), {
  ssr: false,
});

interface IProps {
  gameId: GameId;
  userId?: PlayerId;
  gameType: string;
}

const Game = ({ gameId, gameType, userId }: IProps) => {
  console.log("debug: game.tsx: gameId", gameId, gameType, userId);
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
