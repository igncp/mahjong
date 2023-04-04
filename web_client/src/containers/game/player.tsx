import { useEffect, useState } from "react";

import Header from "src/containers/common/header";
import { HttpClient } from "src/lib/http-client";
import {
  GameId,
  PlayerId,
  TUserLoadGameResponse,
} from "src/lib/mahjong-service";

interface IProps {
  gameId: GameId;
  userId: PlayerId;
}

const Game = ({ gameId, userId }: IProps) => {
  const [serviceGameSummary, setServiceGame] =
    useState<TUserLoadGameResponse | null>(null);

  useEffect(() => {
    // TODO: Improve this with rxjs
    let disconnectSocket = () => {};

    (async () => {
      const [game, disconnect] = await Promise.all([
        HttpClient.userLoadGame(gameId, {
          player_id: userId,
        }),
        () => () => null,
      ]);

      setServiceGame(game);

      disconnectSocket = disconnect;
    })();

    return () => {
      disconnectSocket();
    };
  }, [gameId]);

  if (!serviceGameSummary) return null;

  return (
    <main>
      <Header />
      <p>Game ID: {serviceGameSummary.game_summary.id}</p>
      <p>Score: {serviceGameSummary.game_summary.score[userId]}</p>
    </main>
  );
};

export default Game;
