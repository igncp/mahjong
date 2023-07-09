import { EditOutlined } from "@ant-design/icons";
import { tokenObserver } from "mahjong_sdk/dist/auth";
import {
  DashboardQueryResponse,
  queryDashboardUserQuery,
} from "mahjong_sdk/dist/graphql/dashboard-user-query";
import { HttpClient } from "mahjong_sdk/dist/http-client";
import Head from "next/head";
import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { first } from "rxjs";

import Button from "src/ui/common/button";
import Card from "src/ui/common/card";
import Input from "src/ui/common/input";
import List from "src/ui/common/list";
import Space from "src/ui/common/space";
import Title from "src/ui/common/title";

import { SiteUrls } from "../lib/site/urls";
import styles from "./dashboard-player.module.scss";
import PageContent from "./page-content";

type TProps = {
  userId: string;
};

const DashboardUser = ({ userId }: TProps) => {
  const { t } = useTranslation();
  const [dashboardQueryResponse, setDashboardQueryResponse] =
    useState<DashboardQueryResponse | null>(null);
  const [editName, setEditName] = useState(false);
  const [nameInput, setNameInput] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const router = useRouter();

  useEffect(() => {
    const subscription = queryDashboardUserQuery().subscribe({
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

  const { player, playerGamesIds, playerTotalScore } = dashboardQueryResponse;

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
          data-name="display-name"
          level={2}
          onClick={() => {
            setNameInput(player.name);
            setEditName(true);
          }}
          style={{ cursor: "pointer", margin: "10px 0" }}
        >
          <span data-name="display-name-content">{player.name}</span>{" "}
          <EditOutlined style={{ fontSize: "16px" }} /> (
          {t("dashboard.userPoints", "{{count}} points", {
            count: playerTotalScore,
          })}
          )
        </Title>
      )}
      <List
        bordered
        className={styles.list}
        dataSource={playerGamesIds}
        renderItem={(gameId) => (
          <div className={styles.listItem} data-name="existing-game">
            <Link href={SiteUrls.playerGame(gameId, userId)}>{gameId}</Link>
          </div>
        )}
      />
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
          {t("dashboard.newGame")}
        </Button>
      </div>
    </PageContent>
  );
};

export default DashboardUser;
