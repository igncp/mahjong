import { Tile } from "mahjong_sdk/dist/core";
import { getTileInfo } from "mahjong_sdk/dist/tile-content";
import React from "react";
import { Text, View } from "react-native";

type Props = {
  tile: Tile;
};

export const TileImg = ({ tile }: Props) => {
  const [, tileTitle] = getTileInfo(tile) || [];

  return (
    <View>
      <Text>{tileTitle}</Text>
    </View>
  );
};
