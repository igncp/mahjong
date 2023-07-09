import { ServiceGameSummary, TSocketWrapper } from "mahjong_sdk/dist/core";
import { HttpClient } from "mahjong_sdk/dist/http-client";
import {
  ModelServiceGameSummary,
  ModelState,
} from "mahjong_sdk/dist/service-game-summary";
import React, { useCallback, useEffect, useRef, useState } from "react";
import { Button, ScrollView, StyleSheet, Text, View } from "react-native";

import { TileImg } from "../ui/tile-img";

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

  const [game, setGame] = gameState;

  const gameId = game.game_summary.id;
  const userId = game.game_summary.player_id;
  const serviceGameMRef = useRef<ModelServiceGameSummary | null>(null);

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
    <ScrollView bounces={false}>
      <Text>Game ID: {game.game_summary.id}</Text>
      <Text>Player: {game.game_summary.player_id}</Text>
      <Text>
        Current turn: {turnPlayer.name}
        {turnPlayer.id === player.id ? " (it's you)" : ""}
      </Text>
      <View>
        {gameState[0].game_summary.board.map((tileId) => {
          const tile = serviceGameM.getTile(tileId);

          return <TileImg key={tileId} tile={tile} />;
        })}
      </View>
      <View>
        <Text>Hand: ({hand.length})</Text>
        <View style={handStyles.list}>
          {handWithoutMelds.map((tile) => (
            <View key={tile.id} style={handStyles.item}>
              <TileImg tile={serviceGameM.getTile(tile.id)} />
            </View>
          ))}
        </View>
      </View>
      <Button onPress={onDashboardClick} title="Dashboard" />
    </ScrollView>
  );
};

const handStyles = StyleSheet.create({
  item: {
    borderColor: "black",
    borderWidth: 1,
  },
  list: {
    flexDirection: "row",
    flexWrap: "wrap",
    gap: 8,
  },
});
