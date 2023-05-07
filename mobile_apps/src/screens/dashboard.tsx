import { useFocusEffect } from "@react-navigation/native";
import React, { useCallback, useState } from "react";
import { Button, Text, View } from "react-native";
import { first, take, zip } from "rxjs";

import { tokenObserver } from "mahjong_sdk/src/auth";
import {
  TUserGetGamesResponse,
  TUserGetInfoResponse,
} from "mahjong_sdk/src/core";
import { HttpClient } from "mahjong_sdk/src/http-server";

import { useUserId } from "../lib/auth";

interface IProps {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  navigation: any;
}

const useScreenData = () => {
  const userId = useUserId();
  const [gamesIds, setGamesIds] = useState<TUserGetGamesResponse | null>(null);
  const [userInfo, setUserInfo] = useState<TUserGetInfoResponse | null>(null);

  useFocusEffect(
    useCallback(() => {
      if (!userId) {
        return;
      }

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
    }, [userId])
  );

  return {
    gamesIds,
    userId,
    userInfo,
  };
};

export const DashboardScreen = ({ navigation }: IProps) => {
  const { userId, gamesIds, userInfo } = useScreenData();

  const onLogOut = () => {
    tokenObserver.next(null);
  };

  const onCreateGame = useCallback(() => {
    HttpClient.userCreateGame({
      player_id: userId as string,
    })
      .pipe(first())
      .subscribe({
        next: (game) => {
          navigation.push("Game", {
            game,
          });
        },
      });
  }, [navigation, userId]);

  const onGamePress = useCallback(
    (gameId: string) => {
      HttpClient.userLoadGame(gameId, {
        player_id: userId as string,
      })
        .pipe(first())
        .subscribe({
          next: (game) => {
            navigation.push("Game", {
              game,
            });
          },
        });
    },
    [navigation, userId]
  );

  if (!userId) {
    return null;
  }

  return (
    <View>
      <Text>Dashboard</Text>
      {userInfo && <Text>Username: {userInfo.name}</Text>}
      <Text>Games:</Text>
      {gamesIds &&
        gamesIds.map((game) => (
          <Button key={game} onPress={() => onGamePress(game)} title={game} />
        ))}
      <Button onPress={onCreateGame} title="Create game" />
      <Button onPress={onLogOut} title="Log out" />
    </View>
  );
};
