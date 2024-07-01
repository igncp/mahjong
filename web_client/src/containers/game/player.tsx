import { InfoCircleOutlined } from "@ant-design/icons";
import { message } from "antd";
import { useRouter } from "next/router";
import { Fragment, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { first } from "rxjs";

import { SiteUrls } from "src/lib/site/urls";
import { getTileInfo } from "src/lib/tile-info";
import {
  Board,
  GameId,
  PlayerId,
  ServiceGameSummary,
  SetId,
  TUserLoadGameResponse,
  Wind,
} from "src/sdk/core";
import { useIsMobile } from "src/sdk/hooks";
import { HttpClient } from "src/sdk/http-client";
import {
  ModelServiceGameSummary,
  ModelServiceGameSummaryError,
  ModelState,
} from "src/sdk/service-game-summary";
import Alert from "src/ui/common/alert";
import Button from "src/ui/common/button";
import Card from "src/ui/common/card";
import CopyToClipboard from "src/ui/common/copy-to-clipboard";
import Text from "src/ui/common/text";
import Tooltip from "src/ui/common/tooltip";
import { useGameUI } from "src/ui/game/use-game-ui";
import TileImg from "src/ui/tile-img";

import PageContent from "../page-content";
import GameBoard, { BoardPlayer } from "./board";
import styles from "./player.module.scss";

export interface IProps {
  gameId: GameId;
  userId: PlayerId;
}

const Game = ({ gameId, userId }: IProps) => {
  const { i18n, t } = useTranslation();
  const gameState = useState<null | TUserLoadGameResponse>(null);
  const loadingState = useState(false);
  const [messageApi, contextHolder] = message.useMessage();
  const isMobile = useIsMobile();

  const serviceGameMRef = useRef<ModelServiceGameSummary | null>(null);
  const router = useRouter();

  const [serviceGameSummary, setServiceGame] = gameState;
  const [loading] = loadingState;

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

  const { boardDropRef, canDropInBoard, handTilesProps } = useGameUI({
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

  const dealerPlayerId =
    serviceGameSummary.game_summary.players[
      serviceGameSummary.game_summary.round.dealer_player_index
    ];
  const dealerPlayer = serviceGameSummary.players[dealerPlayerId];

  const isDraggingOther = handTilesProps.some((props) => props.hasItemOver);

  const canPassRound = serviceGameSummary.game_summary.board.length === 92;
  const canDrawTile =
    player.id === userId && serviceGameSummary.game_summary.hand.length < 14;

  return (
    <PageContent headerCollapsible={isMobile}>
      {!isMobile && (
        <>
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
        </>
      )}{" "}
      <span ref={boardDropRef}>
        <GameBoard
          activePlayer={turnPlayer.id}
          canDropInBoard={canDropInBoard}
          dealerPlayer={dealerPlayer.id}
          isMobile={isMobile}
          players={
            boardPlayers as [BoardPlayer, BoardPlayer, BoardPlayer, BoardPlayer]
          }
          serviceGameM={serviceGameM}
          serviceGameSummary={serviceGameSummary}
        />
      </span>
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
              <Button
                disabled={loading || !canDrawTile}
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
              {!serviceGameSummary.settings.ai_enabled && (
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
              )}
              <Button
                disabled={loading}
                onClick={() => {
                  serviceGameM.sayMahjong();
                }}
              >
                {t("game.sayMahjong")}
              </Button>
            </div>
          );
        })()}
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
      </div>
      {contextHolder}
    </PageContent>
  );
};

export default Game;
