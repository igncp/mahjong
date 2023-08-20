import { Tile } from "mahjong_sdk/dist/core";
import { getTileInfo } from "mahjong_sdk/dist/tile-content";
import React from "react";
import { Image, StyleSheet, Text, View } from "react-native";

import { getTileImageName, imageNameToImport } from "../lib/assets";

type Props = {
  tile: Tile;
};

const styles = StyleSheet.create({
  image: {
    height: 130,
    width: 100,
  },
});

export const TileImg = ({ tile }: Props) => {
  const [, tileTitle] = getTileInfo(tile) || [];
  const tileImg = getTileImageName(tile);

  return (
    <View>
      <Text>{tileTitle}</Text>
      <Image source={imageNameToImport[tileImg]} style={styles.image} />
    </View>
  );
};
