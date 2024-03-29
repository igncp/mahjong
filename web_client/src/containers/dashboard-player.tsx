import { EditOutlined, PlusCircleOutlined } from "@ant-design/icons";
import dayjs from "dayjs";
import Head from "next/head";
import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { first } from "rxjs";
import { string } from "zod";

import { tokenObserver } from "src/sdk/auth";
import type { TUserDashboardResponse } from "src/sdk/core";
import { HttpClient } from "src/sdk/http-client";
import Button from "src/ui/common/button";
import Card from "src/ui/common/card";
import Input from "src/ui/common/input";
import Modal from "src/ui/common/modal";
import Space from "src/ui/common/space";
import Table from "src/ui/common/table";
import Title from "src/ui/common/title";

import { SiteUrls } from "../lib/site/urls";
import styles from "./dashboard-player.module.scss";
import PageContent from "./page-content";

export type TProps = {
  userId: string;
};

const simpleFormatDate = (timestamp: string): string => {
  const timestampNum = Number(timestamp);

  if (isNaN(timestampNum)) return "-";

  const day = dayjs(timestampNum);

  return day.format("YYYY-MM-DD HH:mm:ss");
};

const DashboardUser = ({ userId }: TProps) => {
  const { t } = useTranslation();
  const [dashboardQueryResponse, setDashboardQueryResponse] =
    useState<null | TUserDashboardResponse>(null);
  const [editName, setEditName] = useState(false);
  const [nameInput, setNameInput] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedGameId, setSelectGameId] = useState("");

  const router = useRouter();

  useEffect(() => {
    const subscription = HttpClient.getUserDashboard().subscribe({
      error: () => {
        tokenObserver.next(null);
        subscription.unsubscribe();
      },
      next: (newQueryResponse) => {
        setDashboardQueryResponse(newQueryResponse);
      },
    });

    return () => {
      subscription.unsubscribe();
    };
  }, []);

  if (!dashboardQueryResponse) return null;

  const {
    player,
    player_games: playerGames,
    player_total_score: playerTotalScore,
  } = dashboardQueryResponse;

  const isSaveNameDisabled = !nameInput || isLoading;

  const onSaveNameSubmit = () => {
    if (isSaveNameDisabled) return;
    setIsLoading(true);

    HttpClient.userPatchInfo(userId, {
      name: nameInput,
    })
      .pipe(first())
      .subscribe({
        error: () => {
          tokenObserver.next(null);
        },
        next: (newUser) => {
          setIsLoading(false);

          setDashboardQueryResponse({
            ...dashboardQueryResponse,
            player: {
              ...dashboardQueryResponse.player,
              name: newUser.name,
            },
          });
          setEditName(false);
        },
      });
  };

  const onGameClick = (gameId: string) => {
    setSelectGameId(gameId);
    setIsModalOpen(true);
  };

  return (
    <PageContent>
      <Head>
        <title>{t("page.dashboard.title", "Mahjong Dashboard")}</title>
      </Head>
      {editName ? (
        <Space>
          <Card title={t("dashboard.editName", "Edit the name")}>
            <Space direction="vertical">
              <Input
                data-name="display-name-input"
                onChange={(e) => {
                  setNameInput(e.target.value);
                }}
                onPressEnter={onSaveNameSubmit}
                placeholder={
                  t("dashboard.name.placeholder", "The new name") as string
                }
                value={nameInput}
              />
              <Space>
                <Button
                  data-name="display-name-button"
                  disabled={isSaveNameDisabled}
                  onClick={onSaveNameSubmit}
                  type="primary"
                >
                  {t("dashboard.save", "Save")}
                </Button>
                <Button
                  onClick={() => {
                    setEditName(false);
                  }}
                  type="dashed"
                >
                  {t("dashboard.cancel", "Cancel")}
                </Button>
              </Space>
            </Space>
          </Card>
        </Space>
      ) : (
        <Title
          className={styles.displayName}
          data-name="display-name"
          level={2}
          style={{ margin: "10px 0" }}
        >
          <span
            data-name="display-name-content"
            onClick={() => {
              setNameInput(player.name);
              setEditName(true);
            }}
            style={{
              alignItems: "center",
              cursor: "pointer",
              display: "flex",
              gap: 10,
            }}
          >
            {player.name}
            <EditOutlined rev="" style={{ fontSize: "16px" }} />
          </span>{" "}
          (
          {t("dashboard.userPoints", "{{count}} points", {
            count: playerTotalScore,
          })}
          )
          {!!dashboardQueryResponse?.auth_info &&
            (() => {
              const { auth_info } = dashboardQueryResponse;
              const { provider, username } = auth_info;

              if (!provider || !username) return null;

              const [text, link] = (() => {
                switch (provider) {
                  case "Email": {
                    const isEmail = string()
                      .email()
                      .safeParse(username).success;

                    return [
                      isEmail
                        ? username
                        : `${t(
                            "dashboard.authUsername",
                            "Auth username"
                          )}: ${username}`,
                      isEmail ? `mailto:${username}` : null,
                    ];
                  }
                  case "Github":
                    return [
                      `${username}@github`,
                      `https://github.com/${username}`,
                    ];
                  case "Anonymous":
                    return [];
                  default:
                    return [];
                }
              })();

              if (!text) return null;

              return (
                <small
                  onClick={(e) => {
                    e.stopPropagation();
                  }}
                >
                  {link ? (
                    <Link href={link} rel="noreferrer" target="_blank">
                      {text}
                    </Link>
                  ) : (
                    text
                  )}
                </small>
              );
            })()}
        </Title>
      )}
      <div className={styles.newGameButton}>
        <Button
          data-name="create-game-button"
          onClick={() => {
            const playerNums = [
              t("dashboard.playerNum1", "1"),
              t("dashboard.playerNum2", "2"),
              t("dashboard.playerNum3", "3"),
              t("dashboard.playerNum4", "4"),
            ];

            HttpClient.userCreateGame({
              ai_player_names: Array.from({ length: 4 }).map((_, i) =>
                t("dashboard.defaultPlayerName", "Player {{number}}", {
                  number: playerNums[i],
                })
              ),
              player_id: userId,
            })
              .pipe(first())
              .subscribe({
                next: (game) => {
                  router.push(
                    SiteUrls.playerGame(game.game_summary.id, userId)
                  );
                },
              });
          }}
        >
          {t("dashboard.newGame")} <PlusCircleOutlined rev="" />
        </Button>
      </div>
      <Table
        className={styles.table}
        columns={[
          {
            dataIndex: "id",
            key: "id",
            render: (text) => (
              <div data-name="existing-game" onClick={() => onGameClick(text)}>
                {text.slice(0, 12)}...
              </div>
            ),
            responsive: ["xs"],
            title: t("dashboard.table.id", "ID"),
          },
          {
            dataIndex: "id",
            key: "id",
            render: (text) => (
              <div data-name="existing-game" onClick={() => onGameClick(text)}>
                {text.slice(0, 6)}...
              </div>
            ),
            responsive: ["md"],
            title: t("dashboard.table.id", "ID"),
          },
          {
            key: "updated_at",
            render: (record) => (
              <>
                <b>{simpleFormatDate(record.updated_at)}</b>
                <br />
                {simpleFormatDate(record.created_at)}
              </>
            ),
            responsive: ["xs"],
            title: `${t("dashboard.table.played")} / ${t(
              "dashboard.table.created"
            )}`,
          },
          {
            dataIndex: "updated_at",
            key: "updated_at",
            render: (text) => simpleFormatDate(text),
            responsive: ["sm"],
            title: t("dashboard.table.played", "Last played at"),
          },
          {
            dataIndex: "created_at",
            key: "created_at",
            render: (text) => simpleFormatDate(text),
            responsive: ["sm"],
            title: t("dashboard.table.created", "Created at"),
          },
        ]}
        dataSource={playerGames.map((game) => ({ ...game, key: game.id }))}
        onRow={(record) => ({
          onClick: () => onGameClick(record.id),
        })}
      />
      <Modal
        footer={[
          <Button
            key="yes"
            onClick={() => {
              router.push(SiteUrls.playerGame(selectedGameId, userId));
            }}
            type="primary"
          >
            {t("dashboard.yes", "Yes")}
          </Button>,
          <Button
            key="no"
            onClick={() => {
              setIsModalOpen(false);
            }}
          >
            {t("dashboard.no", "No")}
          </Button>,
        ]}
        onCancel={() => {
          setIsModalOpen(false);
        }}
        open={isModalOpen}
        title={t("dashboard.openGame", "Open this game?") as string}
      >
        <p>{selectedGameId}</p>
      </Modal>
    </PageContent>
  );
};

export default DashboardUser;
