import { ServiceGameSummary, TSocketWrapper } from "mahjong_sdk/dist/core";
import { HttpClient } from "mahjong_sdk/dist/http-client";
import {
  ModelServiceGameSummary,
  ModelState,
} from "mahjong_sdk/dist/service-game-summary";
import React, {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { useTranslation } from "react-i18next";
import { Button, Pressable, Text, View } from "react-native";

import LanguagePicker from "../containers/language-picker";
import { TileImg } from "../ui/tile-img";
import { styles } from "./game.styles";

interface IProps {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  navigation: any;
  route: {
    params: {
      game: ServiceGameSummary;
    };
  };
}

export const GameScreen = ({ navigation, route }: IProps) => {
  const { game: initialGame } = route.params;
  const gameState = useState(initialGame);
  const socketRef = useRef<TSocketWrapper>();
  const loadingState = useState(false);
  const { t } = useTranslation();

  const [game, setGame] = gameState;

  const gameId = game.game_summary.id;
  const userId = game.game_summary.player_id;
  const serviceGameMRef = useRef<ModelServiceGameSummary | null>(null);

  const [serviceGameSummary] = gameState;

  useEffect(() => {
    const socket$ = HttpClient.connectToSocket({
      gameId,
      onMessage: (data) => {
        if ("GameSummaryUpdate" in data) {
          setGame(data.GameSummaryUpdate);
        }
      },
      playerId: userId,
    });

    const socketSub = socket$.subscribe((socket) => {
      socketRef.current = socket;
    });

    return () => {
      socket$.value.close();
      socketSub.unsubscribe();
    };
  }, [gameId, userId, setGame]);

  const onDashboardClick = useCallback(() => {
    navigation.pop();
  }, [navigation]);

  const canDiscardTile = useMemo(() => {
    if (!serviceGameSummary) {
      return false;
    }

    const { hand } = serviceGameSummary.game_summary;

    // This should be from API
    return hand.length === 14;
  }, [serviceGameSummary]);

  serviceGameMRef.current =
    serviceGameMRef.current || new ModelServiceGameSummary();

  const serviceGameM = serviceGameMRef.current;
  serviceGameM.updateStates(
    gameState as ModelState<ServiceGameSummary>,
    loadingState
  );

  const player = serviceGameM.getPlayingPlayer();
  const turnPlayer = serviceGameM.getTurnPlayer();
  const { hand } = gameState[0].game_summary;
  const handWithoutMelds = hand.filter((tile) => !tile.set_id);

  return (
    <View style={styles.wrapper}>
      <View style={styles.turn}>
        <Text>
          {t("game.turn", "Current turn: ")}
          {turnPlayer.name}
          {turnPlayer.id === player.id ? t("game.itsYou", " (it's you)") : ""}
        </Text>
      </View>
      <View>
        <Text>
          {t("game.board", "Board: ")}({hand.length})
        </Text>
        <View style={styles.list}>
          {gameState[0].game_summary.board.map((tileId) => {
            const tile = serviceGameM.getTile(tileId);

            return (
              <View key={tileId} style={styles.item}>
                <TileImg tile={tile} />
              </View>
            );
          })}
        </View>
      </View>
      <View>
        <Text>
          {t("game.hand", "Hand: ")}({hand.length})
        </Text>
        <View style={styles.list}>
          {handWithoutMelds.map((tile) => (
            <Pressable
              key={tile.id}
              onPress={() => {
                if (canDiscardTile) {
                  serviceGameM.discardTile(tile.id);
                }
              }}
              style={styles.item}
            >
              <TileImg tile={serviceGameM.getTile(tile.id)} />
            </Pressable>
          ))}
        </View>
      </View>
      <Button
        onPress={onDashboardClick}
        title={t("game.dashboard", "Dashboard")}
      />
      <LanguagePicker />
    </View>
  );
};
