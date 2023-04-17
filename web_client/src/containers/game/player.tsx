import { useRouter } from "next/router";
import { useEffect, useRef, useState } from "react";

import Header from "src/containers/common/header";
import { HttpClient } from "src/lib/http-client";
import {
  GameId,
  PlayerId,
  ServiceGameSummary,
  SetId,
  TUserLoadGameResponse,
  TileId,
} from "src/lib/mahjong-service";
import {
  ModelServiceGameSummary,
  ModelState,
} from "src/lib/models/service-game-summary";
import { SiteUrls } from "src/lib/site/urls";
import Button from "src/ui/common/button";
import Card from "src/ui/common/card";
import CopyToClipboard from "src/ui/common/copy-to-clipboard";
import PageContent from "src/ui/common/page-content";
import Space from "src/ui/common/space";
import Text from "src/ui/common/text";
import TileImg from "src/ui/tile-img";

interface IProps {
  gameId: GameId;
  userId: PlayerId;
}

const Game = ({ gameId, userId }: IProps) => {
  const gameState = useState<TUserLoadGameResponse | null>(null);
  const loadingState = useState(false);

  const serviceGameMRef = useRef<ModelServiceGameSummary | null>(null);
  const router = useRouter();

  const [serviceGameSummary, setServiceGame] = gameState;
  const [loading] = loadingState;

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
      ]).catch(() => {
        router.push(SiteUrls.index);

        return [];
      });

      setServiceGame(game);

      disconnectSocket = disconnect || disconnectSocket;
    })();

    return () => {
      disconnectSocket();
    };
  }, [gameId]);

  if (!serviceGameSummary) return null;

  serviceGameMRef.current =
    serviceGameMRef.current || new ModelServiceGameSummary();

  const serviceGameM = serviceGameMRef.current;
  serviceGameM.updateStates(
    gameState as ModelState<ServiceGameSummary>,
    loadingState
  );

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

  const player = serviceGameM.getPlayingPlayer();
  const playerIndex = serviceGameM.getPlayingPlayerIndex();
  const possibleMelds = serviceGameM.getPossibleMelds();

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const onAIEnabledChanged = (event: any) => {
    const aiEnabled = event.target.value === "enabled";

    serviceGameM.setAIEnabled(aiEnabled);
  };

  return (
    <main style={{ width: "100vw" }}>
      <Header />
      <PageContent>
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
        <p
          style={{
            alignItems: "center",
            display: "flex",
            flexWrap: "wrap",
          }}
        >
          Board:{" "}
          <span
            style={{
              display: "inline-flex",
              flexDirection: "row",
              flexWrap: "wrap",
            }}
          >
            {serviceGameSummary.game_summary.board.map(
              (tileId, tileIndex, tiles) => {
                const isDiscardedTile =
                  tiles.length === tileIndex + 1 &&
                  typeof serviceGameSummary.game_summary.round
                    .discarded_tile === "number";
                const tile = serviceGameM.getTile(tileId);

                return (
                  <span
                    key={tileId}
                    {...(isDiscardedTile
                      ? {
                          onClick: () => {
                            serviceGameM.claimTile();
                          },
                          style: {
                            color: "blue",
                            cursor: "pointer",
                          },
                        }
                      : {})}
                  >
                    <TileImg tile={tile} />
                  </span>
                );
              }
            )}
          </span>
        </p>
        <p
          style={{
            alignItems: "center",
            display: "flex",
            flexWrap: "wrap",
          }}
        >
          Hand: ({hand.length})
          {handWithoutMelds.map((handTile) => (
            <span
              key={handTile.id}
              onClick={() => {
                if (canDiscardTile) {
                  serviceGameM.discardTile(handTile.id);
                }
              }}
              style={{
                color: canDiscardTile ? "black" : "gray",
                cursor: canDiscardTile ? "pointer" : "default",
              }}
            >
              <TileImg tile={serviceGameM.getTile(handTile.id)} />
            </span>
          ))}
        </p>
        <ul>
          {Array.from(setsIds).map((setId) => {
            const setTiles = hand.filter((tile) => tile.set_id === setId);
            const isConcealed = setTiles.every((tile) => tile.concealed);

            return (
              <li
                key={setId}
                style={{
                  alignItems: "center",
                  display: "flex",
                  gap: "5px",
                }}
              >
                - Meld:<b>{` ${isConcealed ? "concealed" : "open"} `}</b>
                {setTiles.map((tile) => (
                  <span key={tile.id}>
                    <TileImg tile={serviceGameM.getTile(tile.id)} />
                  </span>
                ))}
                {isConcealed && (
                  <Button
                    disabled={loading}
                    onClick={() => {
                      serviceGameM.breakMeld(setId);
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
                disabled={
                  loading ||
                  possibleMeld.tiles.filter(
                    (tileId) =>
                      serviceGameSummary.game_summary.hand.find(
                        (handTile) => handTile.id === tileId
                      ) === undefined
                  ).length !== 0
                }
                onClick={async () => {
                  serviceGameM.createMeld(possibleMeld.tiles);
                }}
              >
                Create meld
              </Button>
            </li>
          ))}
        </ul>
        <Text>Other players</Text>
        <ul>
          {serviceGameSummary.game_summary.players.map(
            (playerId, playerIndex) => {
              const player = serviceGameSummary.players[playerId];

              if (playerId === userId) return null;

              const playerHand =
                serviceGameSummary.game_summary.other_hands[playerId];
              const melds = playerHand.visible.reduce((meldsInner, tile) => {
                if (tile.set_id) {
                  meldsInner[tile.set_id] = meldsInner[tile.set_id] || [];
                  meldsInner[tile.set_id].push(tile.id);
                }

                return meldsInner;
              }, {} as Record<string, TileId[]>);
              const meldsSets = Object.keys(melds).sort();

              return (
                <li key={playerId}>
                  {player.name} (
                  {serviceGameSummary.game_summary.score[playerId]}){" "}
                  {playerIndex ===
                  serviceGameSummary.game_summary.round.player_index
                    ? " *"
                    : ""}
                  {meldsSets.length > 0 && (
                    <ul>
                      {meldsSets.map((meldSetId) => {
                        const meldTiles = melds[meldSetId];

                        return (
                          <li key={meldSetId}>
                            Visible meld:{" "}
                            {meldTiles.map((tileId) => (
                              <span key={tileId}>
                                {serviceGameM.getTileString(tileId)}
                              </span>
                            ))}
                          </li>
                        );
                      })}
                    </ul>
                  )}
                </li>
              );
            }
          )}
        </ul>
        <Text>Actions:</Text>
        <Space wrap>
          <Button
            disabled={loading}
            onClick={async () => {
              const gameSummary = await HttpClient.userDrawTile(gameId, {
                game_version: serviceGameSummary.game_summary.version,
                player_id: userId,
              });

              setServiceGame(gameSummary);
            }}
          >
            Draw tile
          </Button>
          <Button
            disabled={loading}
            onClick={async () => {
              const newGame = await HttpClient.userMovePlayer(gameId, {
                player_id: userId,
              });
              setServiceGame(newGame);
            }}
          >
            Next turn
          </Button>
          <Button
            disabled={loading}
            onClick={() => {
              serviceGameM.sortHands();
            }}
          >
            Sort hand
          </Button>
          <Button
            disabled={loading}
            onClick={async () => {
              const { service_game_summary: newGame } =
                await HttpClient.userContinueAI(gameId, {
                  player_id: userId,
                });

              setServiceGame(newGame);
            }}
          >
            Continue AI
          </Button>
          <Button
            disabled={loading}
            onClick={() => {
              serviceGameM.sayMahjong();
            }}
          >
            Say Mahjong
          </Button>
          <Card>
            AI:{" "}
            <form style={{ display: "inline-block" }}>
              <label style={{ marginRight: "10px" }}>
                Enabled
                <input
                  checked={serviceGameSummary.ai_enabled}
                  name="ai_enabled"
                  onChange={onAIEnabledChanged}
                  type="radio"
                  value={"enabled"}
                />
              </label>
              <label>
                Disabled
                <input
                  checked={!serviceGameSummary.ai_enabled}
                  name="ai_enabled"
                  onChange={onAIEnabledChanged}
                  type="radio"
                  value={"disabled"}
                />
              </label>
            </form>
          </Card>
        </Space>
      </PageContent>
    </main>
  );
};

export default Game;
