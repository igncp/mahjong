import { CloseCircleOutlined } from "@ant-design/icons";
import { message } from "antd";
import Alert from "antd/es/alert/Alert";
import type { RoundValidationError } from "bindings/RoundValidationError";
import { useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";

import { OffscreenGame, ScoringRule, Wind } from "pkg/web_lib";
import { getDeck } from "src/sdk/pkg-wrapper";

import { getTileInfo } from "src/lib/tile-info";
import Button from "src/ui/common/button";
import Title from "src/ui/common/title";
import TileImg from "src/ui/tile-img";

import PageContent from "./page-content";

const OffscreenGamePage = () => {
  const [hasRendered, setHasRendered] = useState(false);
  const { i18n } = useTranslation();
  const offscreenGameRef = useRef<null | OffscreenGame>(null);
  const [messageApi, contextHolder] = message.useMessage();

  const [, triggerRerender] = useState(0);

  const rerender = () => {
    triggerRerender((prev) => prev + 1);
  };

  // Player form
  const [newPlayerName, setNewPlayerName] = useState("");
  const [newPlayerWind, setNewPlayerWind] = useState<null | Wind>(null);
  const [newPlayerScore, setNewPlayerScore] = useState("");
  const [newPlayerIsDealer, setNewPlayerIsDealer] = useState(false);
  const [isEditing, setIsEditing] = useState("");

  const [isCalculatingScore, setIsCalculatingScore] = useState<false | number>(
    false,
  );

  const newPlayerNameRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    setHasRendered(true);
  }, []);

  useEffect(() => {
    if (!isEditing) {
      setNewPlayerName("");
      setNewPlayerScore("");
      setNewPlayerIsDealer(false);
      setNewPlayerWind(null);
    }
  }, [isEditing]);

  useEffect(() => {
    if (typeof window === "undefined") return;

    const savedGame = localStorage.getItem("offscreen-game");

    if (!savedGame) return;

    try {
      const game = OffscreenGame.deserialize(savedGame);

      offscreenGameRef.current = game;
    } catch {
      localStorage.removeItem("offscreen-game");
    }
  }, []);

  useEffect(() => {
    if (typeof window === "undefined") return;

    if (!offscreenGameRef.current) return;

    localStorage.setItem(
      "offscreen-game",
      offscreenGameRef.current.serialize(),
    );
  });

  if (!hasRendered) {
    return null;
  }

  if (!offscreenGameRef.current) {
    offscreenGameRef.current = new OffscreenGame();
  }

  const game = offscreenGameRef.current as OffscreenGame;
  const dateCreatedTimestamp = Number(game.date_created);
  const dateCreated = new Date(dateCreatedTimestamp).toLocaleString();

  return (
    <>
      <PageContent contentStyle={{ marginTop: "20px" }}>
        <Title level={1}>Offscreen Game</Title>
        <div style={{ display: "flex" }}>
          <Alert
            description="This section is Work In Progress, it is still not translated and it will change soon"
            message="Info"
            showIcon
            type="info"
          />
        </div>
        <div>
          <Button
            onClick={() => {
              const newGame = new OffscreenGame();

              offscreenGameRef.current = newGame;

              rerender();
            }}
          >
            Reset Game
          </Button>
        </div>
        <p>Date created: {dateCreated}</p>
        {isEditing && (
          <div className="flex flex-col gap-[20px]">
            <div className="flex flex-row flex-wrap gap-[20px]">
              <input
                className="px-[4px]"
                onChange={(e) => setNewPlayerName(e.target.value)}
                placeholder="Name"
                ref={newPlayerNameRef}
                type="text"
                value={newPlayerName}
              />
              <input
                onChange={(e) => setNewPlayerScore(e.target.value)}
                placeholder="Score"
                type="text"
                value={newPlayerScore}
              />
              <label className="align-left">
                Wind{" "}
                <select
                  onChange={(e) => {
                    setNewPlayerWind(Number(e.target.value) as Wind);
                  }}
                  value={newPlayerWind as Wind}
                >
                  <option value={Wind.East}>East</option>
                  <option value={Wind.South}>South</option>
                  <option value={Wind.West}>West</option>
                  <option value={Wind.North}>North</option>
                </select>
              </label>
              <label>
                Dealer{" "}
                <input
                  checked={newPlayerIsDealer}
                  onChange={(e) => setNewPlayerIsDealer(e.target.checked)}
                  type="checkbox"
                />
              </label>
            </div>
            <div className="flex flex-row gap-[20px]">
              <Button
                onClick={() => {
                  setIsEditing("");
                }}
              >
                Close
              </Button>
              <Button
                disabled={!newPlayerName || !newPlayerScore}
                onClick={() => {
                  const playerId = isEditing;

                  const existingPlayer = game.players.find(
                    (player) => player.id === playerId,
                  );

                  if (!existingPlayer) return;

                  existingPlayer.name = newPlayerName;

                  game.set_player(playerId, existingPlayer);

                  if (newPlayerIsDealer) {
                    game.set_dealer(playerId);
                  }

                  game.update_player_score(
                    playerId,
                    parseInt(newPlayerScore, 10) || 0,
                  );

                  if (typeof newPlayerWind === "number") {
                    game.set_wind(playerId, newPlayerWind);
                  }

                  setIsEditing("");
                }}
              >
                Save
              </Button>
            </div>
          </div>
        )}
        <table>
          <thead>
            <tr>
              <th>Player</th>
              <th>Score</th>
              <th>Dealer</th>
              <th>Wind</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {game.players.map((player) => {
              const isDealer = game.is_dealer(player.id);

              const wind = game.get_wind(player.id);

              return (
                <tr key={player.id}>
                  <td>{player.name}</td>
                  <td className="text-right">
                    {game.get_player_score(player.id)}
                  </td>
                  <td className="text-center">{isDealer ? "☑" : "☐"}</td>
                  <td className="text-center">
                    {(() => {
                      switch (wind) {
                        case Wind.East:
                          return "East";
                        case Wind.South:
                          return "South";
                        case Wind.West:
                          return "West";
                        case Wind.North:
                          return "North";
                        default:
                          wind satisfies never;

                          return "-";
                      }
                    })()}
                  </td>
                  <td className="text-center">
                    <Button
                      onClick={() => {
                        setNewPlayerName(player.name);

                        const playerId = player.id;
                        const score = game.get_player_score(playerId);
                        const playerWind = game.get_wind(playerId);

                        setNewPlayerScore(score.toString());
                        setNewPlayerIsDealer(isDealer);
                        setNewPlayerWind(playerWind);
                        setIsEditing(player.id);
                      }}
                    >
                      Edit
                    </Button>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
        <div>
          {isCalculatingScore === false ? (
            <Button
              onClick={() => {
                setIsCalculatingScore(0);
              }}
            >
              Calculate Round Score
            </Button>
          ) : (
            <Button
              onClick={() => {
                setIsCalculatingScore(false);
              }}
            >
              Close
            </Button>
          )}
        </div>
        <div>
          {(() => {
            if (isCalculatingScore === false) return null;

            const player = game.players[isCalculatingScore];
            const playerId = player.id;
            const deck = getDeck();
            const availableTiles = Array.from(game.available_tiles_for_round);
            const selectingHand = game.selecting_hand(playerId);
            const selectingHandTiles = selectingHand.tiles;
            const setsIds = selectingHand.sets_ids.sort();

            return (
              <div className="flex flex-col gap-[20px]">
                <div>
                  <div>
                    Hand of: {player.name} ({selectingHandTiles.length} tiles)
                  </div>
                  <div className="flex flex-row flex-wrap">
                    {selectingHandTiles.map((handTile) => {
                      const tile = deck[handTile.id];

                      return (
                        <div
                          className="align-center m-[10px] flex flex-col justify-center gap-[10px] border border-[#aaa] p-[10px]"
                          key={handTile.id}
                        >
                          <div className="flex flex-row justify-end">
                            <CloseCircleOutlined
                              className="cursor-pointer"
                              onClick={() => {
                                game.select_tile_for_round(
                                  playerId,
                                  handTile.id,
                                );

                                rerender();
                              }}
                            />
                          </div>
                          <TileImg
                            className="align-center flex justify-center"
                            size={100}
                            tile={tile}
                          />
                          <Button
                            onClick={() => {
                              handTile.set_concealed(!handTile.concealed, game);
                              rerender();
                            }}
                          >
                            {handTile.concealed ? "Concealed" : "Visible"}
                          </Button>
                          <div>
                            Meld:{" "}
                            {!!setsIds.length && (
                              <select
                                onChange={(ev) => {
                                  handTile.set_set_id(
                                    ev.target.value || undefined,
                                    game,
                                  );

                                  rerender();
                                }}
                                value={handTile.set_id}
                              >
                                <option value="">None</option>
                                {setsIds.map((setId) => (
                                  <option key={setId} value={setId}>
                                    {setId}
                                  </option>
                                ))}
                              </select>
                            )}
                            <Button
                              onClick={() => {
                                const lastSetId =
                                  setsIds[setsIds.length - 1] ||
                                  `${isCalculatingScore}_0`;

                                const [, setIdVal] = lastSetId.split("_");
                                const setIdNum = parseInt(setIdVal, 10);

                                const newSetId = `${isCalculatingScore}_${
                                  setIdNum + 1
                                }`;

                                handTile.set_set_id(newSetId, game);
                                rerender();
                              }}
                            >
                              Add
                            </Button>
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>
                <div className="flex flex-row gap-[20px]">
                  {isCalculatingScore !== 0 && (
                    <Button
                      onClick={() => {
                        setIsCalculatingScore(isCalculatingScore - 1);
                      }}
                    >
                      Previous
                    </Button>
                  )}
                  <Button
                    onClick={() => {
                      if (isCalculatingScore === 3) {
                        const validResult = game.get_is_valid_round();

                        if (
                          !validResult.is_valid ||
                          !validResult.winner_player
                        ) {
                          const errorMessage = (() => {
                            const errorStr = validResult.error_message_data();

                            if (!errorStr) return null;

                            return JSON.parse(errorStr) as RoundValidationError;
                          })();

                          if (errorMessage === "NoHandMahjong") {
                            messageApi.error(
                              "The round is not valid, check every player tiles",
                            );
                          }

                          return;
                        }

                        const winnerPlayer = game.players.find(
                          (p) => p.id === validResult.winner_player,
                        );

                        if (!winnerPlayer) return;

                        const result = game.update_score(winnerPlayer.id);

                        messageApi.success(
                          `The player ${winnerPlayer.name} won the round with ${result.score} point(s)`,
                        );

                        // eslint-disable-next-line no-console
                        console.log(
                          "Rules:",
                          (result.rules as ScoringRule[])
                            .map((rule) => {
                              switch (rule) {
                                case ScoringRule.AllFlowers:
                                  return "All Flowers";
                                case ScoringRule.AllInTriplets:
                                  return "All In Triplets";
                                case ScoringRule.AllSeasons:
                                  return "All Seasons";
                                case ScoringRule.BasePoint:
                                  return "Base Point";
                                case ScoringRule.CommonHand:
                                  return "Common Hand";
                                case ScoringRule.GreatDragons:
                                  return "Great Dragons";
                                case ScoringRule.LastWallTile:
                                  return "Last Wall Tile";
                                case ScoringRule.NoFlowersSeasons:
                                  return "No Flowers Seasons";
                                case ScoringRule.SeatFlower:
                                  return "Seat Flower";
                                case ScoringRule.SeatSeason:
                                  return "Seat Season";
                                case ScoringRule.SelfDraw:
                                  return "Self Draw";
                                default:
                                  rule satisfies never;

                                  return "";
                              }
                            })
                            .filter(Boolean)
                            .join(", "),
                        );

                        setIsCalculatingScore(false);

                        return;
                      }

                      setIsCalculatingScore(isCalculatingScore + 1);
                    }}
                  >
                    {isCalculatingScore === 3 ? "Calculate" : "Next"}
                  </Button>
                </div>
                <div>
                  <div>Choose the tiles:</div>
                  <div className="mt-[20px] flex flex-row flex-wrap gap-[20px]">
                    {availableTiles.map((tileId) => {
                      const tile = deck[tileId];
                      const tileInfo = getTileInfo(tile, i18n);

                      if (!tileInfo) return null;

                      return (
                        <div key={tileId}>
                          <TileImg
                            className="align-center flex cursor-pointer justify-center"
                            onClick={() => {
                              game.select_tile_for_round(playerId, tileId);
                              rerender();
                            }}
                            size={100}
                            tile={tile}
                          />
                        </div>
                      );
                    })}
                  </div>
                </div>
              </div>
            );
          })()}
        </div>
      </PageContent>
      {contextHolder}
    </>
  );
};

export default OffscreenGamePage;
