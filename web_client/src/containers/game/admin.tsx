import { useRouter } from "next/router";
import { Fragment, useEffect, useState } from "react";

import { ModelServiceGame } from "src/lib/models/service-game";
import { SiteUrls } from "src/lib/site/urls";
import { SetId, TAdminGetGameResponse } from "src/sdk/core";
import { HttpClient } from "src/sdk/http-client";
import Button from "src/ui/common/button";
import CopyToClipboard from "src/ui/common/copy-to-clipboard";

import PageContent from "../page-content";

export interface IProps {
  gameId: string;
}

const Game = ({ gameId }: IProps) => {
  const router = useRouter();
  const [serviceGame, setServiceGame] = useState<null | TAdminGetGameResponse>(
    null
  );

  useEffect(() => {
    const socket$ = HttpClient.connectToSocket({
      gameId,
      onMessage: (data) => {
        if ("GameUpdate" in data) {
          setServiceGame(data.GameUpdate);
        }
      },
    });

    HttpClient.adminGetGame(gameId).subscribe({
      error: () => {
        router.push(SiteUrls.index);
        return [];
      },
      next: (game) => {
        setServiceGame(game);
      },
    });

    return () => {
      socket$.value.close();
    };
  }, [gameId]);

  if (!serviceGame) return null;

  const serviceGameM = new ModelServiceGame(serviceGame);
  const currentPlayer = serviceGameM.getCurrentPlayer();
  const possibleMelds = serviceGameM.getPossibleMelds();

  return (
    <PageContent>
      <p>
        Game ID: <CopyToClipboard text={gameId} />
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
                onClick={async () => {
                  const lastTile =
                    serviceGame.game.table.draw_wall[
                      serviceGame.game.table.draw_wall.length - 1
                    ];

                  const newGame = await HttpClient.adminDrawWallSwapTiles(
                    gameId,
                    {
                      tile_id_a: tileId,
                      tile_id_b: lastTile,
                    }
                  );

                  setServiceGame(newGame);
                }}
                style={{ cursor: !isLast ? "pointer" : "default" }}
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
              {hand.length}) [Score: {serviceGameM.getPlayerScore(player.id)}] (
              <CopyToClipboard text={player.id} />)
            </p>
            <ul>
              <li>
                {handWithoutMelds.map((handTile) => (
                  <span
                    key={handTile.id}
                    onClick={async () => {
                      if (canDiscardTile) {
                        const newGame = await HttpClient.adminDiscardTile(
                          gameId,
                          { tile_id: handTile.id }
                        );

                        setServiceGame(newGame);
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
                          const newHand = await HttpClient.adminBreakMeld(
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
                      const hand = await HttpClient.adminCreateMeld(gameId, {
                        player_id: player.id,
                        tiles: playerPossibleMeld.tiles,
                      });

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
              const newHand = await HttpClient.adminDrawCard(gameId);
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
              const newGame = await HttpClient.adminMovePlayer(gameId);
              setServiceGame(newGame);
            }}
          >
            Pass turn
          </Button>
        </li>
        <li>
          <Button
            onClick={async () => {
              const hands = await HttpClient.adminSortHands(gameId);
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
              const newGame = await HttpClient.adminAIContinue(gameId);

              setServiceGame(newGame.service_game);
            }}
          >
            Run AI
          </Button>
          <Button
            onClick={async () => {
              const newGame = await HttpClient.adminAIContinue(gameId, {
                draw: false,
              });

              setServiceGame(newGame.service_game);
            }}
          >
            Run AI without draw
          </Button>
        </li>
        <li>
          <Button
            onClick={async () => {
              const game = await HttpClient.adminSayMahjong(gameId, {
                player_id: currentPlayer.id,
              });

              setServiceGame(game);
            }}
          >
            Say Mahjong
          </Button>
        </li>
      </ul>
    </PageContent>
  );
};

export default Game;
