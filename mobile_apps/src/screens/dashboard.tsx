import { useFocusEffect } from "@react-navigation/native";
import dayjs from "dayjs";
import { tokenObserver } from "mahjong_sdk/dist/auth";
import {
  DashboardQueryResponse,
  queryDashboardUserQuery,
} from "mahjong_sdk/dist/graphql/dashboard-user-query";
import { HttpClient } from "mahjong_sdk/dist/http-client";
import React, { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";
import { Button, Text, View } from "react-native";
import { first } from "rxjs";

import LanguagePicker from "../containers/language-picker";
import { useIsConnected } from "../lib/net";
import { styles } from "./dashboard.styles";

interface IProps {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  navigation: any;
}

const simpleFormatDate = (timestamp: string): string => {
  const timestampNum = Number(timestamp);

  if (isNaN(timestampNum)) {
    return "-";
  }

  const day = dayjs(timestampNum);

  return day.format("YYYY-MM-DD HH:mm:ss");
};

const useScreenData = () => {
  const [dashboardQueryResponse, setDashboardQueryResponse] =
    useState<DashboardQueryResponse | null>(null);

  useFocusEffect(
    useCallback(() => {
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
    }, [])
  );

  const { playerGames, player, playerTotalScore } =
    dashboardQueryResponse || {};

  return {
    player,
    playerGames,
    playerTotalScore,
  };
};

export const DashboardScreen = ({ navigation }: IProps) => {
  const { player, playerGames, playerTotalScore } = useScreenData();
  const { t } = useTranslation();

  const onLogOut = () => {
    tokenObserver.next(null);
  };

  const gameRoute = t("router.game");
  const isConnected = useIsConnected();

  const onCreateGame = useCallback(() => {
    if (!player) {
      return;
    }

    HttpClient.userCreateGame({
      player_id: player.id,
    })
      .pipe(first())
      .subscribe({
        next: (game) => {
          navigation.push(gameRoute, {
            game,
          });
        },
      });
  }, [navigation, player, gameRoute]);

  const onGamePress = useCallback(
    (gameId: string) => {
      if (!player) {
        return;
      }
      HttpClient.userLoadGame(gameId, {
        player_id: player.id,
      })
        .pipe(first())
        .subscribe({
          next: (game) => {
            navigation.push(gameRoute, {
              game,
            });
          },
        });
    },
    [navigation, player, gameRoute]
  );

  if (!player?.id) {
    return null;
  }

  return (
    <View style={styles.wrapper}>
      <Text>
        {t("dashboard.username", "Username:")}{" "}
        <Text style={styles.highlight}>{player.name}</Text>
      </Text>
      <Text>
        {t("dashboard.totalScore", "Total score:")} {playerTotalScore}
      </Text>
      <Button
        disabled={!isConnected}
        onPress={onCreateGame}
        title={t("dashboard.create", "Create game")}
      />
      {playerGames?.length ? (
        <View style={styles.gamesList}>
          <Text>{t("dashboard.games", "Games:")}</Text>
          {playerGames.map((game) => (
            <Button
              color="darkgreen"
              disabled={!isConnected}
              key={game.id}
              onPress={() => onGamePress(game.id)}
              title={[
                game.id,
                t("dashboard.lastPlayed", "Last played: ") +
                  simpleFormatDate(game.updatedAt),
              ].join("\n")}
            />
          ))}
        </View>
      ) : (
        <Text>{t("dashboard.noGames")}</Text>
      )}
      <Button onPress={onLogOut} title={t("dashboard.logout")} />
      <LanguagePicker />
    </View>
  );
};
