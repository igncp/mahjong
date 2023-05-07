import { Tile } from "mahjong_sdk/src/core";
import { getTileInfo } from "mahjong_sdk/src/tile-content";

import Tooltip from "./common/tooltip";

const TileImg = ({ tile }: { tile: Tile }) => {
  const [image, title] = getTileInfo(tile) || [];

  if (!image) {
    return null;
  }

  return (
    <Tooltip title={title as string}>
      <img src={image} style={{ height: "50px", width: "50px" }} />
    </Tooltip>
  );
};

export default TileImg;
