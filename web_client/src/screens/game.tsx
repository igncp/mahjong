import { useEffect, useState } from "react";
import Header from "src/containers/common/header";
import { HttpClient } from "src/lib/http-client";
import { TAdminGetGameResponse } from "src/lib/mahjong-service";

interface IProps {
  gameId: string;
  gameType: string;
}

const Game = ({ gameId, gameType }: IProps) => {
  const [game, setGame] = useState<TAdminGetGameResponse | null>(null);

  useEffect(() => {
    (async () => {
      const httpClient = HttpClient.singleton();
      const game = await httpClient.adminGetGame(gameId);

      setGame(game);
    })();
  }, [gameId, gameType]);

  if (!game) return null;

  return (
    <main>
      <Header />
      <p>
        Game ID: {gameId} {gameType}
      </p>
      <p>Game name: {game.game.name}</p>
      <p>Draw Wall: {game.game.table.draw_wall.length}</p>
      {Object.keys(game.players).map((playerId) => {
        const player = game.players[playerId];
        return <p key={player.id}>{player.name}</p>;
      })}
    </main>
  );
};

export default Game;
