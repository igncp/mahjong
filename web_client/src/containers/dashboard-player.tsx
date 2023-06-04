import { EditOutlined } from "@ant-design/icons";
import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { first, take, zip } from "rxjs";

import { tokenObserver } from "mahjong_sdk/src/auth";
import {
  TUserGetGamesResponse,
  TUserGetInfoResponse,
} from "mahjong_sdk/src/core";
import { HttpClient } from "mahjong_sdk/src/http-server";
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
  const [gamesIds, setGamesIds] = useState<TUserGetGamesResponse | null>(null);
  const [userInfo, setUserInfo] = useState<TUserGetInfoResponse | null>(null);
  const [editName, setEditName] = useState(false);
  const [nameInput, setNameInput] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const router = useRouter();

  useEffect(() => {
    const subscription = zip(
      HttpClient.userGetGames({
        player_id: userId,
      }),
      HttpClient.userGetInfo(userId)
    )
      .pipe(take(1))
      .subscribe({
        error: () => {
          tokenObserver.next(null);
          subscription.unsubscribe();
        },
        next: ([games, user]) => {
          setUserInfo(user || null);
          setGamesIds(games || null);
        },
      });

    return () => {
      subscription.unsubscribe();
    };
  }, []);

  if (!gamesIds || !userInfo) return null;

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

          setUserInfo(newUser || null);
          setEditName(false);
        },
      });
  };

  return (
    <PageContent>
      {editName ? (
        <Space>
          <Card title={t("dashboard.editName", "Edit the name")}>
            <Space direction="vertical">
              <Input
                onChange={(e) => {
                  setNameInput(e.target.value);
                }}
                onPressEnter={onSaveNameSubmit}
                placeholder="The new name"
                value={nameInput}
              />
              <Space>
                <Button
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
          level={2}
          onClick={() => {
            setNameInput(userInfo.name);
            setEditName(true);
          }}
          style={{ cursor: "pointer", margin: "10px 0" }}
        >
          {userInfo.name} <EditOutlined style={{ fontSize: "16px" }} /> (
          {t("dashboard.userPoints", "{{count}} points", {
            count: userInfo.total_score,
          })}
          )
        </Title>
      )}
      <List
        bordered
        className={styles.list}
        dataSource={gamesIds}
        renderItem={(gameId) => (
          <div className={styles.listItem}>
            <Link href={SiteUrls.playerGame(gameId, userId)}>{gameId}</Link>
          </div>
        )}
      />
      <div className={styles.newGameButton}>
        <Button
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
