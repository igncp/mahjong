import { InfoCircleOutlined } from "@ant-design/icons";
import { message } from "antd";
import {
  Board,
  GameId,
  GameSettings,
  PlayerId,
  ServiceGameSummary,
  SetId,
  TUserLoadGameResponse,
  Wind,
} from "mahjong_sdk/dist/core";
import { HttpClient } from "mahjong_sdk/dist/http-client";
import {
  ModelServiceGameSummary,
  ModelServiceGameSummaryError,
  ModelState,
} from "mahjong_sdk/dist/service-game-summary";
import { useRouter } from "next/router";
import { Fragment, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { first } from "rxjs";

import { SiteUrls } from "src/lib/site/urls";
import { getTileInfo } from "src/lib/tile-info";
import Alert from "src/ui/common/alert";
import Button from "src/ui/common/button";
import Card from "src/ui/common/card";
import CopyToClipboard from "src/ui/common/copy-to-clipboard";
import List, { ListItem } from "src/ui/common/list";
import Select, { SelectOption } from "src/ui/common/select";
import Text from "src/ui/common/text";
import Tooltip from "src/ui/common/tooltip";
import UserAvatar from "src/ui/common/user-avatar";
import { useGameUI } from "src/ui/game/use-game-ui";
import TileImg from "src/ui/tile-img";

import PageContent from "../page-content";
import GameBoard, { BoardPlayer } from "./board";
import styles from "./player.module.scss";

interface IProps {
  gameId: GameId;
  userId: PlayerId;
}

const convertDiscardWaitMsValue = (value: GameSettings["discard_wait_ms"]) => {
  if (value === null) return "none";
  if (value === 1000) return "1s";
  if (value === 10000) return "10s";
  if (value === 60000) return "1m";
  if (value === -1) return "block";
  return "none";
};

const Game = ({ gameId, userId }: IProps) => {
  const { t, i18n } = useTranslation();
  const gameState = useState<TUserLoadGameResponse | null>(null);
  const loadingState = useState(false);
  const [messageApi, contextHolder] = message.useMessage();

  const serviceGameMRef = useRef<ModelServiceGameSummary | null>(null);
  const router = useRouter();

  const [serviceGameSummary, setServiceGame] = gameState;
  const [loading] = loadingState;

  const autoSortOptions: SelectOption[] = useMemo(
    () => [
      {
        label: t("game.option.yes"),
        value: "yes",
      },
      {
        label: t("game.option.no"),
        value: "no",
      },
    ],
    [t]
  );

  const autoStopDrawMeldOptions: SelectOption[] = useMemo(
    () => [
      {
        label: t("game.option.yes"),
        value: "yes",
      },
      {
        label: t("game.option.no"),
        value: "no",
      },
    ],
    [t]
  );

  const discardWaitMsOptions: SelectOption[] = useMemo(
    () => [
      {
        label: t("game.wait.none"),
        value: "none",
      },
      {
        label: t("game.wait.1sec"),
        value: "1s",
      },
      {
        label: t("game.wait.10sec"),
        value: "10s",
      },
      {
        label: t("game.wait.1min"),
        value: "1m",
      },
      {
        label: t("game.wait.block"),
        value: "block",
      },
    ],
    [t]
  );

  useEffect(() => {
    const socket$ = HttpClient.connectToSocket({
      gameId,
      onMessage: (data) => {
        if ("GameSummaryUpdate" in data) {
          setServiceGame(data.GameSummaryUpdate);
        }
      },
      playerId: userId,
    });

    HttpClient.userLoadGame(gameId, {
      player_id: userId,
    })
      .pipe(first())
      .subscribe({
        error: (error) => {
          console.log("debug: player.tsx: error", error);
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

  serviceGameMRef.current =
    serviceGameMRef.current || new ModelServiceGameSummary();
  const serviceGameM = serviceGameMRef.current;

  useEffect(() => {
    if (!serviceGameM) return;

    const subscription = serviceGameM.errorEmitter$.subscribe({
      next: (error) => {
        setTimeout(() => {
          if (error === ModelServiceGameSummaryError.INVALID_SAY_MAHJONG) {
            messageApi.open({
              content: t(
                "game.error.invalidMahjong",
                "You can't say Mahjong now"
              ),
              type: "error",
            });
          }
        }, 100);
      },
    });

    return () => {
      subscription.unsubscribe();
    };
  }, [serviceGameM, messageApi]);

  const getCanDiscardTile = () => {
    if (!serviceGameSummary) return false;

    const { hand } = serviceGameSummary.game_summary;

    // This should be from API
    return hand.length === 14;
  };

  serviceGameM.updateStates(
    gameState as ModelState<ServiceGameSummary>,
    loadingState
  );

  const boardPlayers = useMemo(
    () =>
      serviceGameSummary?.game_summary.players.map((player) => {
        const playerSummary = serviceGameSummary?.players[player];
        return {
          id: playerSummary.id,
          name: playerSummary.name,
        };
      }),
    [serviceGameSummary?.game_summary.players.join("")]
  );

  const { boardDropRef, handTilesProps, canDropInBoard } = useGameUI({
    getCanDiscardTile,
    serviceGameM,
    serviceGameSummary,
  });

  const windToText: Record<Wind, string> = {
    [Wind.East]: t("game.wind.east"),
    [Wind.North]: t("game.wind.north"),
    [Wind.South]: t("game.wind.south"),
    [Wind.West]: t("game.wind.west"),
  };

  if (!serviceGameSummary) return null;

  const { hand } = serviceGameSummary.game_summary;
  const handWithoutMelds = serviceGameM.getPlayerHandWithoutMelds();

  const setsIds = hand.reduce((acc, tile) => {
    if (tile.set_id) {
      acc.add(tile.set_id);
    }
    return acc;
  }, new Set<SetId>());

  const player = serviceGameM.getPlayingPlayer();
  const turnPlayer = serviceGameM.getTurnPlayer();
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
        case "block":
          return -1;
        default:
          return null;
      }
    })();

    serviceGameM.setGameSettings({
      ...serviceGameSummary.settings,
      discard_wait_ms: msValue,
    });
  };

  const onAutoSortChange = (value: string) => {
    const boolValue = value === "yes";

    serviceGameM.setGameSettings({
      ...serviceGameSummary.settings,
      auto_sort: boolValue,
    });
  };

  const onAutoStopDrawMeldChange = (value: string) => {
    const boolValue = value === "yes";

    serviceGameM.setGameSettings({
      ...serviceGameSummary.settings,
      auto_stop_claim_meld: boolValue,
    });
  };

  const dealerPlayerId =
    serviceGameSummary.game_summary.players[
      serviceGameSummary.game_summary.round.dealer_player_index
    ];
  const dealerPlayer = serviceGameSummary.players[dealerPlayerId];

  const isDraggingOther = handTilesProps.some((props) => props.hasItemOver);

  const canPassRound = serviceGameSummary.game_summary.board.length === 92;

  return (
    <PageContent>
      <Text style={{ marginTop: "20px" }}>
        <b style={{ fontSize: "20px" }}>{player.name}</b> (
        <span
          style={{
            fontWeight:
              serviceGameSummary.game_summary.score[userId] > 0 ? 700 : 400,
          }}
        >
          {t("game.points", "{{count}} points", {
            count: serviceGameSummary.game_summary.score[userId],
          })}
        </span>
        )
        {process.env.NODE_ENV !== "production" && (
          <>
            {" "}
            <CopyToClipboard text={userId} />
          </>
        )}
      </Text>
      <Text>
        {t("game.currentWind")}{" "}
        <b>{windToText[serviceGameSummary.game_summary.round.wind]}</b>,{" "}
        {t("game.currentDealer")} <b>{dealerPlayer.name}</b>
      </Text>
      <span ref={boardDropRef}>
        <Card
          bodyStyle={{ padding: 0 }}
          title={`${t("game.board")} (${
            serviceGameSummary.game_summary.board.length
          } / 92)`}
        >
          <GameBoard
            activePlayer={turnPlayer.id}
            canDropInBoard={canDropInBoard}
            dealerPlayer={dealerPlayer.id}
            players={
              boardPlayers as [
                BoardPlayer,
                BoardPlayer,
                BoardPlayer,
                BoardPlayer
              ]
            }
            serviceGameM={serviceGameM}
            serviceGameSummary={serviceGameSummary}
          />
        </Card>
      </span>
      <div className={styles.topInfo}>
        <Alert
          message={
            <>
              {t("game.currentTurn")}:{" "}
              <b>
                {turnPlayer.name}
                {turnPlayer.id === player.id ? ` (${t("game.itsYou")})` : ""}
              </b>
            </>
          }
          style={{ display: "inline-block" }}
          type="info"
        />
        {(() => {
          const board = gameState[0]?.game_summary.board as Board;
          const lastTile = board[board?.length - 1];
          const tile = serviceGameM.getTile(lastTile);
          const claimTileTitle = tile ? getTileInfo(tile, i18n)?.[1] : null;
          const disabled = !gameState[0]?.game_summary.round.discarded_tile;

          return (
            <div className={styles.boardButtons}>
              <Button
                disabled={disabled}
                onClick={() => {
                  HttpClient.userMovePlayer(gameId, {
                    player_id: userId,
                  }).subscribe({
                    next: (newGame) => {
                      setServiceGame(newGame);
                    },
                  });
                }}
                type="primary"
              >
                {t("game.passTurn")}
              </Button>
              <Button
                disabled={disabled}
                onClick={() => {
                  serviceGameM.claimTile();
                }}
                type="primary"
              >
                {t("game.claimTile")}
                {claimTileTitle ? `: ${claimTileTitle}` : ""}
              </Button>
              {canPassRound && (
                <Button
                  onClick={() => {
                    serviceGameM.passRound();
                  }}
                  type="primary"
                >
                  {t("game.passRound", "Pass round")}
                </Button>
              )}
            </div>
          );
        })()}
      </div>
      <div>
        <Card
          title={
            <>
              <Tooltip title={t("game.discardInfo")}>
                {t("game.hand")}: {hand.length} <InfoCircleOutlined rev="" />
              </Tooltip>
            </>
          }
        >
          <div className={styles.handTiles}>
            {handWithoutMelds.map((handTile, handTileIndex) => {
              const { hasItemOver, ...handTileProps } =
                handTilesProps[handTileIndex];

              return (
                <Fragment key={`${handTile.id}-${handTileIndex}`}>
                  <TileImg
                    {...handTileProps}
                    isDraggingOther={isDraggingOther}
                    paddingLeft={hasItemOver ? 10 : 0}
                  />
                </Fragment>
              );
            })}
          </div>
        </Card>
      </div>
      <div className={styles.smallGrid}>
        {Array.from(setsIds).map((setId) => {
          const setTiles = hand.filter((tile) => tile.set_id === setId);
          const isConcealed = setTiles.every((tile) => tile.concealed);

          return (
            <Card
              className={styles.cardSmall}
              key={setId}
              title={
                <>
                  {t("game.meld.title")}:
                  <b>{` ${
                    isConcealed ? t("game.meld.concealed") : t("game.meld.open")
                  } `}</b>
                </>
              }
            >
              <div className={styles.meldTile}>
                <div>
                  {setTiles.map((tile) => (
                    <span key={tile.id}>
                      <TileImg tile={serviceGameM.getTile(tile.id)} />
                    </span>
                  ))}
                </div>
                {isConcealed && (
                  <Button
                    disabled={loading}
                    onClick={() => {
                      serviceGameM.breakMeld(setId);
                    }}
                  >
                    {t("game.breakMeld")}
                  </Button>
                )}
              </div>
            </Card>
          );
        })}
        {possibleMelds.map((possibleMeld, index) => {
          const meldByClaiming =
            possibleMeld.tiles.filter(
              (tileId) =>
                serviceGameSummary.game_summary.hand.find(
                  (handTile) => handTile.id === tileId
                ) === undefined
            ).length !== 0;

          return (
            <Card
              className={styles.cardSmall}
              key={index}
              title={
                <Text>
                  <b>{t("game.possibleMeld")}</b>
                  {meldByClaiming && ` (${t("game.claiming")})`}
                </Text>
              }
            >
              <div className={styles.possibleMeld}>
                <div>
                  {possibleMeld.tiles.map((tileId) => {
                    const tile = serviceGameM.getTile(tileId);

                    return (
                      <span key={tileId}>
                        <TileImg tile={tile} />
                      </span>
                    );
                  })}
                </div>
                <Button
                  disabled={loading || meldByClaiming}
                  onClick={() => {
                    serviceGameM.createMeld(possibleMeld.tiles);
                  }}
                  type="primary"
                >
                  {t("game.createMeld")}
                </Button>
              </div>
            </Card>
          );
        })}
        {gameState[0]?.game_summary.players.reduce((acc, playerId) => {
          const playerHand = gameState[0]?.game_summary.other_hands[playerId];

          if (!playerHand?.visible) {
            return acc;
          }

          const sets = new Set(playerHand.visible.map((tile) => tile.set_id));
          const otherPlayer = gameState[0]?.players[playerId];

          Array.from(sets)
            .sort()
            .forEach((setId) => {
              const tiles = playerHand.visible
                .filter((tile) => tile.set_id === setId)
                .map((tile) => tile.id);

              acc.push(
                <Card
                  className={styles.cardSmall}
                  key={setId}
                  title={
                    <Text>
                      <b>
                        {t("game.visibleMeld", {
                          player: otherPlayer?.name,
                        })}
                      </b>
                    </Text>
                  }
                >
                  <div className={styles.possibleMeld}>
                    <div>
                      {tiles.map((tileId) => {
                        const tile = serviceGameM.getTile(tileId);

                        return (
                          <span key={tileId}>
                            <TileImg tile={tile} />
                          </span>
                        );
                      })}
                    </div>
                  </div>
                </Card>
              );
            });

          return acc;
        }, [] as React.ReactElement[])}
        <Card
          className={styles.cardSmall}
          title={
            <Text>
              <b>{t("game.actions")}</b>
            </Text>
          }
        >
          <div className={styles.actionsPanel}>
            <Button
              disabled={loading}
              onClick={() => {
                const sendErrorMessage = () => {
                  // TODO: Move to the error emitter
                  messageApi.open({
                    content: t("game.error.invalidDraw"),
                    type: "error",
                  });
                };

                HttpClient.userDrawTile(gameId, {
                  game_version: serviceGameSummary.game_summary.version,
                  player_id: userId,
                }).subscribe({
                  error: () => {
                    sendErrorMessage();
                  },
                  next: (gameSummary) => {
                    setServiceGame(gameSummary);
                  },
                });
              }}
            >
              {t("game.drawTile")}
            </Button>
            <Button
              disabled={loading}
              onClick={() => {
                serviceGameM.sortHands();
              }}
            >
              {t("game.sortHand")}
            </Button>
            <Button
              disabled={loading}
              onClick={() => {
                HttpClient.userContinueAI(gameId, {
                  player_id: userId,
                }).subscribe({
                  next: ({ service_game_summary: newGame }) => {
                    setServiceGame(newGame);
                  },
                });
              }}
            >
              {t("game.continueAI")}
            </Button>
            <Button
              disabled={loading}
              onClick={() => {
                serviceGameM.sayMahjong();
              }}
            >
              {t("game.sayMahjong")}
            </Button>
          </div>
        </Card>

        <Card
          className={`${styles.cardSmall} ${styles.cardOtherPlayers}`}
          title={
            <Text>
              <b>{t("game.otherPlayers")}</b>
            </Text>
          }
        >
          <List
            dataSource={serviceGameSummary.game_summary.players.filter(
              (p) => p !== userId
            )}
            renderItem={(playerId) => {
              const player = serviceGameSummary.players[playerId];

              return (
                <ListItem>
                  <div className={styles.otherPlayer}>
                    <UserAvatar />
                    <Text
                      style={{
                        alignItems: "center",
                        display: "flex",
                        marginLeft: "10px",
                      }}
                    >
                      {player.name} ({" "}
                      {t("game.points", {
                        count: serviceGameSummary.game_summary.score[playerId],
                      })}
                      )
                    </Text>
                  </div>
                </ListItem>
              );
            }}
            style={{ background: "white" }}
          />
        </Card>
        <Card className={styles.cardSmall} title={t("game.settings.title")}>
          <form className={styles.cardContentSettings}>
            <div className={styles.settingsFormInner}>
              <Text>
                <b>{t("game.AI.title")}</b>:{" "}
                <label style={{ marginRight: "10px" }}>
                  {t("game.AI.enabled")}
                  <input
                    checked={serviceGameSummary.settings.ai_enabled}
                    name="ai_enabled"
                    onChange={onAIEnabledChanged}
                    type="radio"
                    value={"enabled"}
                  />
                </label>
                <label>
                  {t("game.AI.disabled")}
                  <input
                    checked={!serviceGameSummary.settings.ai_enabled}
                    name="ai_enabled"
                    onChange={onAIEnabledChanged}
                    type="radio"
                    value={"disabled"}
                  />
                </label>
              </Text>
              <Text>{t("game.blockTime.desc")}: </Text>
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
              <Text>{t("game.autoSort")}</Text>
              <Select
                defaultValue={
                  serviceGameSummary.settings.auto_sort ? "yes" : "no"
                }
                disabled={false}
                onChange={onAutoSortChange}
                options={autoSortOptions}
                style={{ width: 120 }}
              />
              <Text>{t("game.autoStopDrawMeld")}</Text>
              <Select
                defaultValue={
                  serviceGameSummary.settings.auto_stop_claim_meld
                    ? "yes"
                    : "no"
                }
                disabled={false}
                onChange={onAutoStopDrawMeldChange}
                options={autoStopDrawMeldOptions}
                style={{ width: 120 }}
              />
            </div>
          </form>
        </Card>
      </div>
      {contextHolder}
    </PageContent>
  );
};

export default Game;
