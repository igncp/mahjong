import { useEffect, useState } from "react";

import Header from "src/containers/common/header";
import { HttpClient } from "src/lib/http-client";
import {
  GameId,
  ModelServiceGameSummary,
  PlayerId,
  SetId,
  TUserLoadGameResponse,
} from "src/lib/mahjong-service";
import Button from "src/ui/common/button";
import CopyToClipboard from "src/ui/common/copy-to-clipboard";

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
        HttpClient.connectToSocket({
          gameId,
          onMessage: (data) => {
            if (data.GameSummaryUpdate) {
              setServiceGame(data.GameSummaryUpdate);
            }
          },
          playerId: userId,
        }),
      ]);

      setServiceGame(game);

      disconnectSocket = disconnect;
    })();

    return () => {
      disconnectSocket();
    };
  }, [gameId]);

  if (!serviceGameSummary) return null;

  const serviceGameM = new ModelServiceGameSummary(serviceGameSummary);
  console.log("debug: player.tsx: serviceGameSummary", serviceGameSummary);

  const { hand } = serviceGameSummary.game_summary;
  const canDiscardTile = hand.length === 14;
  const handWithoutMelds = hand.filter((tile) => !tile.set_id);

  const setsIds = hand.reduce((acc, tile) => {
    if (tile.set_id) {
      acc.add(tile.set_id);
    }
    return acc;
  }, new Set<SetId>());

  const player =
    serviceGameSummary.players[serviceGameSummary.game_summary.player_id];
  const playerIndex = serviceGameSummary.game_summary.players.findIndex(
    (player) => player === userId
  );
  const possibleMelds = serviceGameM.getPossibleMelds();

  return (
    <main>
      <Header />
      <p>
        Game ID: <CopyToClipboard text={serviceGameSummary.game_summary.id} />
      </p>
      <p>
        <b>{player.name}</b>{" "}
        {playerIndex === serviceGameSummary.game_summary.round.player_index
          ? " *"
          : ""}{" "}
        ({serviceGameSummary.game_summary.score[userId]}){" "}
        <CopyToClipboard text={userId} />
      </p>
      <p>
        Draw Wall:{" "}
        <span
          style={{
            display: "inline-flex",
            flexDirection: "row",
            flexWrap: "wrap",
            gap: "10px",
          }}
        >
          {serviceGameSummary.game_summary.draw_wall_count}
        </span>
      </p>
      <p>
        Board:{" "}
        <span
          style={{
            display: "inline-flex",
            flexDirection: "row",
            flexWrap: "wrap",
            gap: "10px",
          }}
        >
          {serviceGameSummary.game_summary.board.map((tileId) => (
            <span key={tileId}>{serviceGameM.getTileString(tileId)}</span>
          ))}
        </span>
      </p>
      <p>Hand: ({hand.length})</p>
      {handWithoutMelds.map((handTile) => (
        <span
          key={handTile.id}
          onClick={async () => {
            if (canDiscardTile) {
              const serviceGame = await HttpClient.userDiscardTile(gameId, {
                player_id: userId,
                tile_id: handTile.id,
              });
              setServiceGame(serviceGame);
            }
          }}
          style={{
            color: canDiscardTile ? "black" : "gray",
            cursor: canDiscardTile ? "pointer" : "default",
          }}
        >
          {serviceGameM.getTileString(handTile.id)}
        </span>
      ))}
      <ul>
        {Array.from(setsIds).map((setId) => {
          const setTiles = hand.filter((tile) => tile.set_id === setId);
          const isConcealed = setTiles.every((tile) => tile.concealed);

          return (
            <li key={setId}>
              Meld:{` ${isConcealed ? "concealed" : "open"} `}
              {setTiles.map((tile) => (
                <span key={tile.id}>{serviceGameM.getTileString(tile.id)}</span>
              ))}
              {isConcealed && (
                <Button
                  onClick={async () => {
                    const serviceGame = await HttpClient.userBreakMeld(gameId, {
                      player_id: userId,
                      set_id: setId,
                    });
                    setServiceGame(serviceGame);
                  }}
                >
                  Break meld
                </Button>
              )}
            </li>
          );
        })}
        {possibleMelds.map((possibleMeld, idx) => (
          <li key={idx}>
            Possible meld:{" "}
            {possibleMeld.tiles.map((tileId) => (
              <span key={tileId}>{serviceGameM.getTileString(tileId)}</span>
            ))}
            <Button
              onClick={async () => {
                const serviceGame = await HttpClient.userCreateMeld(gameId, {
                  player_id: userId,
                  tiles: possibleMeld.tiles,
                });
                setServiceGame(serviceGame);
              }}
            >
              Create meld
            </Button>
          </li>
        ))}
      </ul>
      <p>Other players</p>
      <ul>
        {serviceGameSummary.game_summary.players.map(
          (playerId, playerIndex) => {
            const player = serviceGameSummary.players[playerId];

            if (playerId === userId) return null;

            return (
              <li key={playerId}>
                {player.name} ({serviceGameSummary.game_summary.score[playerId]}
                ){" "}
                {playerIndex ===
                serviceGameSummary.game_summary.round.player_index
                  ? " *"
                  : ""}
              </li>
            );
          }
        )}
      </ul>
      <p>Actions:</p>
      <ul>
        <li>
          <Button
            onClick={async () => {
              const gameSummary = await HttpClient.userDrawTile(gameId, {
                player_id: userId,
              });

              setServiceGame(gameSummary);
            }}
          >
            Draw tile
          </Button>
        </li>
        <li>
          <Button
            onClick={async () => {
              const newGame = await HttpClient.userMovePlayer(gameId, {
                player_id: userId,
              });
              setServiceGame(newGame);
            }}
          >
            Next turn
          </Button>
        </li>
        <li>
          <Button
            onClick={async () => {
              const newGame = await HttpClient.userSortHand(gameId, {
                player_id: userId,
              });
              setServiceGame(newGame);
            }}
          >
            Sort hand
          </Button>
        </li>
      </ul>
    </main>
  );
};

export default Game;
