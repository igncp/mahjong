import { InfoCircleOutlined, LoadingOutlined } from "@ant-design/icons";
import { message } from "antd";
import type { ServiceGameSummary } from "bindings/ServiceGameSummary";
import type { UserGetLoadGameResponse } from "bindings/UserGetLoadGameResponse";
import type { Wind } from "bindings/Wind";
import { useRouter } from "next/router";
import QRCode from "qrcode";
import type { LegacyRef } from "react";
import { Fragment, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { first } from "rxjs";

import { SiteUrls } from "src/lib/site/urls";
import { getTileInfo } from "src/lib/tile-info";
import type { GameId, PlayerId, SetId } from "src/sdk/core";
import { useIsMobile } from "src/sdk/hooks";
import { HttpClient } from "src/sdk/http-client";
import type { ModelState } from "src/sdk/service-game-summary";
import {
  ModelServiceGameSummary,
  ModelServiceGameSummaryError,
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
import type { BoardPlayer } from "./board";
import GameBoard from "./board";
import styles from "./player.module.scss";

export interface IProps {
  gameId: GameId;
  userId: PlayerId;
}

const Game = ({ gameId, userId }: IProps) => {
  const { i18n, t } = useTranslation();
  const gameState = useState<null | UserGetLoadGameResponse>(null);
  const loadingState = useState(false);
  const [messageApi, contextHolder] = message.useMessage();
  const isMobile = useIsMobile();

  const serviceGameMRef = useRef<ModelServiceGameSummary | null>(null);
  const router = useRouter();

  const [serviceGameSummary, setServiceGame] = gameState;
  const [loading] = loadingState;
  const [qrCodeVal, setQRCode] = useState<null | string>(null);

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
          console.error("debug: player.tsx: error", error);
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
  }, [gameId, router, userId, setServiceGame]);

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
                "You can't say Mahjong now",
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
  }, [serviceGameM, messageApi, t]);

  const isWaitingPlayers =
    serviceGameSummary?.game_summary.phase === "WaitingPlayers";

  const joinUrl = isWaitingPlayers
    ? serviceGameM.getShareLink(serviceGameSummary.game_summary.id)
    : null;

  useEffect(() => {
    if (joinUrl) {
      QRCode.toDataURL(joinUrl)
        .then((url) => {
          setQRCode(url);
        })
        .catch((err) => {
          console.error(err);
        });
    }
  }, [joinUrl]);

  const getCanDiscardTile = () => {
    if (!serviceGameSummary) return false;

    const { hand } = serviceGameSummary.game_summary;

    if (!hand?.list) return false;

    // This should be from API
    return hand.list.length === 14;
  };

  serviceGameM.updateStates(
    gameState as ModelState<ServiceGameSummary>,
    loadingState,
  );

  const boardPlayers = useMemo(
    () =>
      serviceGameSummary?.game_summary.players
        .map((player) => {
          const playerSummary = serviceGameSummary?.players[player];

          if (!playerSummary) return null;

          return {
            id: playerSummary.id,
            name: playerSummary.name,
          };
        })
        .filter(Boolean) as BoardPlayer[],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [serviceGameSummary?.game_summary.players.join("")],
  );

  const { boardDropRef, canDropInBoard, handTilesProps } = useGameUI({
    getCanDiscardTile,
    serviceGameM,
    serviceGameSummary,
  });

  const windToText: Record<Wind, string> = {
    East: t("game.wind.east"),
    North: t("game.wind.north"),
    South: t("game.wind.south"),
    West: t("game.wind.west"),
  };

  if (!serviceGameSummary) return null;

  const { bonus_tiles, hand } = serviceGameSummary.game_summary;
  const handWithoutMelds = serviceGameM.getPlayerHandWithoutMelds();

  const setsIds = (hand?.list || []).reduce((acc, tile) => {
    if (tile.set_id) {
      acc.add(tile.set_id);
    }

    return acc;
  }, new Set<SetId>());

  const playingPlayer = serviceGameM.getPlayingPlayer();
  const turnPlayer = serviceGameM.getTurnPlayer();
  const possibleMelds = serviceGameM.getPossibleMelds();

  const dealerPlayerId =
    serviceGameSummary.game_summary.players[
      serviceGameSummary.game_summary.round.dealer_player_index
    ];

  const dealerPlayer = serviceGameSummary.players[dealerPlayerId];

  const isDraggingOther = handTilesProps?.some((props) => props.hasItemOver);

  const canPassRound = serviceGameSummary.game_summary.board.length === 92;

  const canDrawTile =
    !!playingPlayer &&
    playingPlayer.id === userId &&
    serviceGameSummary.game_summary.hand?.list &&
    serviceGameSummary.game_summary.hand.list.length < 14;

  const playerBonusTiles =
    bonus_tiles[serviceGameSummary.game_summary.player_id];

  const isPlaying = serviceGameSummary.game_summary.phase === "Playing";

  return (
    <PageContent headerCollapsible={isMobile}>
      <div className="mt-[20px] hidden md:block" />
      {!isMobile && isPlaying && (
        <>
          <Text>
            <b style={{ fontSize: "20px" }}>{playingPlayer.name}</b> (
            <span
              style={{
                fontWeight:
                  serviceGameSummary.game_summary.score[userId] > 0 ? 700 : 400,
              }}
            >
              {t("game.points", {
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
      <span ref={boardDropRef as unknown as LegacyRef<HTMLSpanElement>}>
        <GameBoard
          activePlayer={turnPlayer?.id}
          canDropInBoard={canDropInBoard}
          dealerPlayer={dealerPlayer?.id}
          isMobile={isMobile}
          players={
            boardPlayers as [BoardPlayer, BoardPlayer, BoardPlayer, BoardPlayer]
          }
          serviceGameM={serviceGameM}
          serviceGameSummary={serviceGameSummary}
        />
      </span>
      {!isWaitingPlayers && !isPlaying && (
        <div className="my-[20px] flex w-full items-center justify-center">
          <LoadingOutlined style={{ fontSize: "150%" }} />
        </div>
      )}
      {isWaitingPlayers &&
        (() => (
          <>
            <div>
              {t(
                "game.waitingPlayers",
                "Waiting for other players to join ...",
              )}
            </div>
            {joinUrl && (
              <div>
                {t("game.copyLink", "Copy this link and share with them:")}{" "}
                {joinUrl}{" "}
                <button
                  className="text-[#0070f3] underline"
                  onClick={() => {
                    navigator.clipboard.writeText(joinUrl);

                    messageApi.info(t("copied", "Copied"));
                  }}
                >
                  {t("copyClipboard", "Copy to clipboard")}
                </button>
              </div>
            )}
            {qrCodeVal && (
              <>
                <div>
                  {t("game.scanQR", "Or tell them to scan this QR code:")}
                </div>
                <img
                  className="w-[400px] max-w-[100%] border-[1px] border-[black]"
                  src={qrCodeVal}
                />
              </>
            )}
          </>
        ))()}
      {isPlaying && (
        <>
          <div>
            <Card
              title={
                hand?.list ? (
                  <Tooltip title={t("game.discardInfo")}>
                    {t("game.hand")}: {hand.list.length}{" "}
                    <InfoCircleOutlined rev="" />
                  </Tooltip>
                ) : null
              }
            >
              <div className="flex flex-wrap items-center [&_img]:mb-[10px] [&_img]:cursor-pointer">
                {(handWithoutMelds?.list || []).map(
                  (handTile, handTileIndex) => {
                    const { hasItemOver, ...handTileProps } =
                      handTilesProps?.[handTileIndex] || {};

                    return (
                      <Fragment key={`${handTile.id}-${handTileIndex}`}>
                        <TileImg
                          {...handTileProps}
                          isDraggingOther={isDraggingOther}
                          paddingLeft={hasItemOver ? 10 : 0}
                        />
                      </Fragment>
                    );
                  },
                )}
              </div>
            </Card>
          </div>
          <div className="flex flex-wrap items-center justify-center gap-[10px] md:justify-normal">
            <Alert
              message={
                <>
                  {t("game.currentTurn")}:{" "}
                  <b>
                    {turnPlayer.name}
                    {turnPlayer.id === playingPlayer.id
                      ? ` (${t("game.itsYou")})`
                      : ""}
                  </b>
                </>
              }
              style={{ display: "inline-block" }}
              type="info"
            />
            {(() => {
              const board = gameState[0]?.game_summary.board;
              const lastTile = board?.[board?.length - 1];

              const tile =
                typeof lastTile === "number"
                  ? serviceGameM.getTile(lastTile)
                  : lastTile;

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
        </>
      )}
      <div className={styles.smallGrid}>
        {Array.from(setsIds).map((setId) => {
          const setTiles = (hand?.list || []).filter(
            (tile) => tile.set_id === setId,
          );

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
              <div className="flex flex-1 flex-col items-center justify-center gap-[10px] py-[10px]">
                <div className="flex flex-row">
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
                (serviceGameSummary.game_summary.hand?.list || []).find(
                  (handTile) => handTile.id === tileId,
                ) === undefined,
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
              <div className="flex flex-1 flex-col items-center justify-center gap-[10px] bg-[white] py-[10px]">
                <div className="inline-flex flex-row">
                  {possibleMeld.tiles.map((tileId) => {
                    const tile = serviceGameM.getTile(tileId);

                    return (
                      <span key={tileId}>
                        <TileImg tile={tile} />
                      </span>
                    );
                  })}
                </div>
                <div>
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
              </div>
            </Card>
          );
        })}
        {gameState[0]?.game_summary.players.reduce((acc, playerId) => {
          const playerHand = gameState[0]?.game_summary.other_hands[playerId];

          if (!playerHand?.visible) {
            return acc;
          }

          const sets = new Set(
            playerHand.visible.list?.map((tile) => tile.set_id) || [],
          );

          const otherPlayer = gameState[0]?.players[playerId];

          Array.from(sets)
            .sort()
            .forEach((setId) => {
              const tiles = playerHand.visible.list
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
                  <div className="flex flex-1 flex-row items-center justify-center gap-[10px]">
                    {tiles.map((tileId) => {
                      const tile = serviceGameM.getTile(tileId);

                      return (
                        <span key={tileId}>
                          <TileImg tile={tile} />
                        </span>
                      );
                    })}
                  </div>
                </Card>,
              );
            });

          return acc;
        }, [] as React.ReactElement[])}
        {!!playerBonusTiles?.length && (
          <Card
            className={styles.cardSmall}
            title={t("game.bonusTiles", {
              count: playerBonusTiles.length,
            })}
          >
            <div className="flex w-max flex-1 flex-row items-center justify-center">
              {(playerBonusTiles || []).map((bonusTile) => (
                <TileImg
                  key={bonusTile}
                  tile={serviceGameM.getTile(bonusTile)}
                />
              ))}
            </div>
          </Card>
        )}
      </div>
      {contextHolder}
    </PageContent>
  );
};

export default Game;
