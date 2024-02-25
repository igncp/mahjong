import { usePreview } from "react-dnd-preview";

import { getTile } from "src/sdk/service-game-summary";
import TileImg from "src/ui/tile-img";

export const DnDPreview = () => {
  const preview = usePreview();
  if (!preview.display) {
    return null;
  }
  const { item, style } = preview;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const tileId = (item as any)?.tileId;
  if (typeof tileId !== "number") {
    return null;
  }

  const tile = getTile(tileId);

  return (
    <div style={style}>
      <TileImg tile={tile} />
    </div>
  );
};
