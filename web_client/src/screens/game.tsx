import { Fragment, useEffect, useState } from "react";

import Header from "src/containers/common/header";
import { HttpClient } from "src/lib/http-client";
import {
  ModelServiceGame,
  SetId,
  TAdminGetGameResponse,
} from "src/lib/mahjong-service";
import Button from "src/ui/common/button";

interface IProps {
  gameId: string;
  gameType: string;
}

const Game = ({ gameId, gameType }: IProps) => {
  const [serviceGame, setServiceGame] = useState<TAdminGetGameResponse | null>(
    null
  );

  useEffect(() => {
    // TODO: Improve this with rxjs
    let disconnectSocket = () => {};

    (async () => {
      const httpClient = HttpClient.singleton();
      const [game, disconnect] = await Promise.all([
        httpClient.adminGetGame(gameId),
        httpClient.connectToSocket({
          gameId,
          onMessage: (data) => {
            if (data.GameUpdate) {
              setServiceGame(data.GameUpdate);
            }
          },
        }),
      ]);

      setServiceGame(game);

      disconnectSocket = disconnect;
    })();

    return () => {
      disconnectSocket();
    };
  }, [gameId, gameType]);

  if (!serviceGame) return null;

  const serviceGameM = new ModelServiceGame(serviceGame, setServiceGame);
  const currentPlayer = serviceGameM.getCurrentPlayer();
  const possibleMelds = serviceGameM.getPossibleMelds();

  return (
    <main>
      <Header />
      <p>
        Game ID: {gameId} {gameType}
      </p>
      <p>Game name: {serviceGame.game.name}</p>
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
          {serviceGame.game.table.draw_wall.map((tileId, tileIndex) => {
            const isLast =
              tileIndex === serviceGame.game.table.draw_wall.length - 1;

            return (
              <span
                key={tileId}
                style={{ cursor: !isLast ? "pointer" : "default" }}
                onClick={async () => {
                  const lastTile =
                    serviceGame.game.table.draw_wall[
                      serviceGame.game.table.draw_wall.length - 1
                    ];

                  const newGame =
                    await HttpClient.singleton().adminDrawWallSwapTiles(
                      gameId,
                      {
                        tile_id_a: tileId,
                        tile_id_b: lastTile,
                      }
                    );

                  setServiceGame(newGame);
                }}
              >
                {serviceGameM.getTileString(tileId)}
              </span>
            );
          })}
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
          {serviceGame.game.table.board.map((tileId) => (
            <span key={tileId}>{serviceGameM.getTileString(tileId)}</span>
          ))}
        </span>
      </p>
      {serviceGame.game.players.map((playerId) => {
        const player = serviceGame.players[playerId];
        const hand = serviceGame.game.table.hands[playerId];
        const canDiscardTile =
          player.id === currentPlayer.id && hand.length === 14;
        const handWithoutMelds = hand.filter((tile) => !tile.set_id);
        const playerPossibleMelds = possibleMelds.filter(
          (p) => p.player_id === player.id
        );

        const setsIds = hand.reduce((acc, tile) => {
          if (tile.set_id) {
            acc.add(tile.set_id);
          }
          return acc;
        }, new Set<SetId>());

        return (
          <Fragment key={playerId}>
            <p>
              {player.name} {player.id === currentPlayer.id ? "*" : ""} (
              {hand.length})
            </p>
            <ul>
              <li>
                {handWithoutMelds.map((handTile) => (
                  <span
                    key={handTile.id}
                    style={{
                      color: canDiscardTile ? "black" : "gray",
                      cursor: canDiscardTile ? "pointer" : "default",
                    }}
                    onClick={async () => {
                      if (canDiscardTile) {
                        const newGame =
                          await HttpClient.singleton().adminDiscardTile(
                            gameId,
                            { tile_id: handTile.id }
                          );

                        setServiceGame(newGame);
                      }
                    }}
                  >
                    {serviceGameM.getTileString(handTile.id)}
                  </span>
                ))}
              </li>
              {Array.from(setsIds).map((setId) => {
                const setTiles = hand.filter((tile) => tile.set_id === setId);
                const isConcealed = setTiles.every((tile) => tile.concealed);

                return (
                  <li key={setId}>
                    Meld:{` ${isConcealed ? "concealed" : "open"} `}
                    {setTiles.map((tile) => (
                      <span key={tile.id}>
                        {serviceGameM.getTileString(tile.id)}
                      </span>
                    ))}
                    {isConcealed && (
                      <Button
                        onClick={async () => {
                          const newHand =
                            await HttpClient.singleton().adminBreakMeld(
                              gameId,
                              {
                                player_id: player.id,
                                set_id: setId,
                              }
                            );

                          serviceGame.game.table.hands[player.id] = newHand;
                          setServiceGame({ ...serviceGame });
                        }}
                      >
                        Break meld
                      </Button>
                    )}
                  </li>
                );
              })}

              {playerPossibleMelds.map((playerPossibleMeld, idx) => (
                <li key={idx}>
                  Possible meld:{" "}
                  {playerPossibleMeld.tiles.map((tileId) => (
                    <span key={tileId}>
                      {serviceGameM.getTileString(tileId)}
                    </span>
                  ))}
                  <Button
                    onClick={async () => {
                      const hand = await HttpClient.singleton().adminCreateMeld(
                        gameId,
                        {
                          player_id: player.id,
                          tiles: playerPossibleMeld.tiles,
                        }
                      );

                      serviceGame.game.table.hands[player.id] = hand;
                      setServiceGame({ ...serviceGame });
                    }}
                  >
                    Create meld
                  </Button>
                </li>
              ))}
            </ul>
          </Fragment>
        );
      })}
      <p>Actions:</p>
      <ul>
        <li>
          <Button
            onClick={async () => {
              const newHand = await HttpClient.singleton().adminDrawCard(
                gameId
              );
              serviceGame.game.table.hands[currentPlayer.id] = newHand;
              setServiceGame({ ...serviceGame });
            }}
          >
            Draw tile
          </Button>
        </li>
        <li>
          <Button
            onClick={async () => {
              const newGame = await HttpClient.singleton().adminMovePlayer(
                gameId
              );
              setServiceGame(newGame);
            }}
          >
            Pass turn
          </Button>
        </li>
        <li>
          <Button
            onClick={async () => {
              const hands = await HttpClient.singleton().adminSortHands(gameId);
              serviceGame.game.table.hands = hands;
              setServiceGame({ ...serviceGame });
            }}
          >
            Sort hands
          </Button>
        </li>
        <li>
          <Button
            onClick={async () => {
              const newGame = await HttpClient.singleton().adminAIContinue(
                gameId
              );

              setServiceGame(newGame.service_game);
            }}
          >
            Run AI
          </Button>
          <Button
            onClick={async () => {
              const newGame = await HttpClient.singleton().adminAIContinue(
                gameId,
                {
                  draw: false,
                }
              );

              setServiceGame(newGame.service_game);
            }}
          >
            Run AI without draw
          </Button>
        </li>
      </ul>
    </main>
  );
};

export default Game;
