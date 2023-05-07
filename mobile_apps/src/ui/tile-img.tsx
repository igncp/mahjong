import React from "react";
import { Text, View } from "react-native";

import { Tile } from "mahjong_sdk/src/core";
import { getTileInfo } from "mahjong_sdk/src/tile-content";

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
