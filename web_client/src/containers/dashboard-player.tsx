import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
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
import List, { ListItem } from "src/ui/common/list";
import PageContent from "src/ui/common/page-content";
import Space from "src/ui/common/space";
import Title from "src/ui/common/title";

import { SiteUrls } from "../lib/site/urls";

type TProps = {
  userId: string;
};

const DashboardUser = ({ userId }: TProps) => {
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
          <Card title="Edit the name">
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
                  Save
                </Button>
                <Button
                  onClick={() => {
                    setEditName(false);
                  }}
                  type="dashed"
                >
                  Cancel
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
          style={{ cursor: "pointer" }}
        >
          {userInfo.name} ({userInfo.total_score} points)
        </Title>
      )}
      <Space>
        <List
          bordered
          dataSource={gamesIds}
          renderItem={(gameId) => (
            <ListItem>
              <Link href={SiteUrls.playerGame(gameId, userId)}>{gameId}</Link>
            </ListItem>
          )}
        />
      </Space>
      <Button
        onClick={() => {
          HttpClient.userCreateGame({
            player_id: userId,
          })
            .pipe(first())
            .subscribe({
              next: (game) => {
                router.push(SiteUrls.playerGame(game.game_summary.id, userId));
              },
            });
        }}
      >
        Create game
      </Button>
    </PageContent>
  );
};

export default DashboardUser;
