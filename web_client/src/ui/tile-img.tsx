import { memo, useMemo } from "react";
import { useDrag } from "react-dnd";
import { useTranslation } from "react-i18next";

import { Tile } from "mahjong_sdk/src/core";
import { useEffectExceptOnMount } from "mahjong_sdk/src/hooks";
import { getTileInfo } from "src/lib/tile-info";

import Tooltip from "./common/tooltip";

type Props = {
  draggableItem?: unknown;
  draggableType?: string;
  LeftDrop?: React.FC;
  onIsDraggingChange?: (isDragging: boolean) => void;
  tile: Tile;
};

const TileImg = ({
  draggableItem,
  draggableType,
  LeftDrop,
  tile,
  onIsDraggingChange,
}: Props) => {
  const { i18n } = useTranslation();
  const [{ isDragging }, dragRef] = useDrag(
    () => ({
      collect: (monitor) => ({
        isDragging: !!monitor.isDragging(),
      }),
      item: draggableItem,
      type: draggableType || "default",
    }),
    [draggableType, draggableItem]
  );
  const [image, title] =
    useMemo(() => getTileInfo(tile, i18n), [i18n.language]) || [];

  useEffectExceptOnMount(() => {
    onIsDraggingChange?.(isDragging);
  }, [isDragging]);

  if (!image) {
    return null;
  }

  const imgEl = (
    <img
      ref={draggableType ? dragRef : undefined}
      src={image}
      style={{
        height: "50px",
        opacity: isDragging ? 0.5 : 1,
        width: isDragging ? 0 : "50px",
      }}
    />
  );

  return (
    <>
      {LeftDrop && !isDragging && <LeftDrop />}
      <Tooltip title={isDragging ? "" : (title as string)}>{imgEl}</Tooltip>
    </>
  );
};

export default memo(TileImg);
