import { useRouter } from "next/router";
import { useEffect, useRef, useState } from "react";

import Header from "src/containers/common/header";
import { HttpClient } from "src/lib/http-client";
import {
  GameId,
  GameSettings,
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
import { lightGreen } from "src/ui/common/colors";
import CopyToClipboard from "src/ui/common/copy-to-clipboard";
import List, { ListItem } from "src/ui/common/list";
import PageContent from "src/ui/common/page-content";
import Select, { SelectOption } from "src/ui/common/select";
import Space from "src/ui/common/space";
import Text from "src/ui/common/text";
import UserAvatar from "src/ui/common/user-avatar";
import TileImg from "src/ui/tile-img";

interface IProps {
  gameId: GameId;
  userId: PlayerId;
}

const discardWaitMsOptions: SelectOption[] = [
  {
    label: "None",
    value: "none",
  },
  {
    label: "1 second",
    value: "1s",
  },
  {
    label: "10 seconds",
    value: "10s",
  },
  {
    label: "1 minute",
    value: "1m",
  },
];

const convertDiscardWaitMsValue = (value: GameSettings["discard_wait_ms"]) => {
  if (value === null) return "none";
  if (value === 1000) return "1s";
  if (value === 10000) return "10s";
  if (value === 60000) return "1m";
  return "none";
};

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
            console.log("debug: player.tsx: data", data);
            if (data.GameSummaryUpdate) {
              setServiceGame(data.GameSummaryUpdate);
            }
          },
          playerId: userId,
        }),
      ]).catch((error) => {
        console.log("debug: player.tsx: error", error);
        router.push(SiteUrls.index);

        return [];
      });

      console.log("debug: player.tsx: game", game);
      setServiceGame(game);

      disconnectSocket = disconnect || disconnectSocket;
    })();

    return () => {
      disconnectSocket();
    };
  }, [gameId]);

  console.log("debug: player.tsx: serviceGameSummary", serviceGameSummary);
  if (!serviceGameSummary) return null;

  serviceGameMRef.current =
    serviceGameMRef.current || new ModelServiceGameSummary();

  const serviceGameM = serviceGameMRef.current;
  serviceGameM.updateStates(
    gameState as ModelState<ServiceGameSummary>,
    loadingState
  );

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

    serviceGameM.setGameSettings({
      ...serviceGameSummary.settings,
      ai_enabled: aiEnabled,
    });
  };

  const onDiscardWaitMsChanged = (value: string) => {
    const msValue = (() => {
      switch (value) {
        case "1s":
          return 1000;
        case "10s":
          return 10000;
        case "1m":
          return 60000;
        default:
          return null;
      }
    })();

    serviceGameM.setGameSettings({
      ...serviceGameSummary.settings,
      discard_wait_ms: msValue,
    });
  };

  console.log("RENDER");

  return (
    <main style={{ width: "100vw" }}>
      <Header />
      <PageContent style={{ paddingTop: "20px" }}>
        <Text>
          <b>{player.name}</b>{" "}
          {playerIndex === serviceGameSummary.game_summary.round.player_index
            ? " *"
            : ""}{" "}
          ({serviceGameSummary.game_summary.score[userId]}){" "}
          <CopyToClipboard text={userId} />
        </Text>
        <Card
          bodyStyle={{ background: lightGreen }}
          title={`Board (${serviceGameSummary.game_summary.board.length} / ${
            serviceGameSummary.game_summary.draw_wall_count +
            serviceGameSummary.game_summary.board.length
          })`}
        >
          <Text
            style={{
              alignItems: "center",
              display: "flex",
              flexWrap: "wrap",
            }}
          >
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
          </Text>
        </Card>
        <Space>
          <Card title={<>Hand: ({hand.length})</>}>
            <Text
              style={{
                alignItems: "center",
                display: "flex",
                flexWrap: "wrap",
              }}
            >
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
            </Text>
          </Card>
        </Space>
        {!!Array.from(setsIds).length && (
          <Space>
            <List
              bordered
              dataSource={Array.from(setsIds).sort((a, b) =>
                a.localeCompare(b)
              )}
              renderItem={(setId) => {
                const setTiles = hand.filter((tile) => tile.set_id === setId);
                const isConcealed = setTiles.every((tile) => tile.concealed);

                return (
                  <ListItem
                    style={{
                      alignItems: "center",
                      display: "flex",
                      gap: "5px",
                      justifyContent: "start",
                    }}
                  >
                    <Text>
                      Meld:<b>{` ${isConcealed ? "concealed" : "open"} `}</b>
                    </Text>
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
                  </ListItem>
                );
              }}
            />
          </Space>
        )}
        {!!possibleMelds.length && (
          <Space>
            <List
              bordered
              dataSource={possibleMelds}
              renderItem={(possibleMeld) => (
                <ListItem
                  style={{
                    alignItems: "center",
                    display: "flex",
                    gap: "10px",
                    justifyContent: "start",
                  }}
                >
                  Possible meld:{" "}
                  <Space>
                    {possibleMeld.tiles.map((tileId) => {
                      const tile = serviceGameM.getTile(tileId);

                      return <TileImg key={tileId} tile={tile} />;
                    })}
                  </Space>
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
                </ListItem>
              )}
            />
          </Space>
        )}
        <div
          style={{
            alignItems: "start",
            display: "flex",
            flexDirection: "row",
            flexWrap: "wrap",
            gap: "10px",
          }}
        >
          <Space>
            <List
              bordered
              dataSource={serviceGameSummary.game_summary.players.filter(
                (p) => p !== userId
              )}
              header={
                <Text>
                  <b>Other players</b>
                </Text>
              }
              renderItem={(playerId) => {
                const player = serviceGameSummary.players[playerId];
                const playerIndex =
                  serviceGameSummary.game_summary.players.indexOf(playerId);

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
                  <ListItem>
                    <Space style={{ minWidth: "150px" }}>
                      <UserAvatar />
                      <Text>
                        {player.name} (
                        {serviceGameSummary.game_summary.score[playerId]}){" "}
                        {playerIndex ===
                        serviceGameSummary.game_summary.round.player_index
                          ? " *"
                          : ""}
                      </Text>
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
                    </Space>
                  </ListItem>
                );
              }}
            />
          </Space>
          <Space>
            <Card title="Actions">
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
              </Space>
            </Card>
          </Space>
          <Space>
            <Card title="Settings">
              <form style={{ display: "inline-block" }}>
                <Space direction="vertical">
                  <Text>
                    Ai:{" "}
                    <label style={{ marginRight: "10px" }}>
                      Enabled
                      <input
                        checked={serviceGameSummary.settings.ai_enabled}
                        name="ai_enabled"
                        onChange={onAIEnabledChanged}
                        type="radio"
                        value={"enabled"}
                      />
                    </label>
                    <label>
                      Disabled
                      <input
                        checked={!serviceGameSummary.settings.ai_enabled}
                        name="ai_enabled"
                        onChange={onAIEnabledChanged}
                        type="radio"
                        value={"disabled"}
                      />
                    </label>
                  </Text>
                  <br />
                  <Text>Block time for next turn pass after discard: </Text>
                  <Select
                    defaultValue={
                      convertDiscardWaitMsValue(
                        serviceGameSummary.settings.discard_wait_ms
                      ) || "none"
                    }
                    disabled={!serviceGameSummary.settings.ai_enabled}
                    onChange={onDiscardWaitMsChanged}
                    options={discardWaitMsOptions}
                    style={{ width: 120 }}
                  />
                </Space>
              </form>
            </Card>
          </Space>
        </div>
      </PageContent>
    </main>
  );
};

export default Game;
