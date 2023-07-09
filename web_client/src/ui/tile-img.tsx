import { Tile } from "mahjong_sdk/dist/core";
import { useEffectExceptOnMount } from "mahjong_sdk/dist/hooks";
import { memo, useMemo } from "react";
import { ConnectDropTarget, useDrag } from "react-dnd";
import { useTranslation } from "react-i18next";

import { getTileInfo } from "src/lib/tile-info";

import Tooltip from "./common/tooltip";

type Props = {
  draggableItem?: unknown;
  draggableType?: string;
  dropRef?: ConnectDropTarget;
  onIsDraggingChange?: (isDragging: boolean) => void;
  paddingLeft?: number;
  isDraggingOther?: boolean;
  tile: Tile;
};

const TileImg = ({
  draggableItem,
  draggableType,
  dropRef,
  onIsDraggingChange,
  paddingLeft,
  tile,
  isDraggingOther,
}: Props) => {
  const { i18n } = useTranslation();
  const [{ isDragging }, dragRef] = useDrag(
    () => ({
      canDrag: !!draggableType,
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
    <span ref={dropRef}>
      <span
        style={{
          display: "inline-block",
          height: "1px",
          transition: "width 0.25s",
          width: `${paddingLeft || 0}px`,
        }}
      />
      <img
        ref={draggableType ? dragRef : undefined}
        src={image}
        style={{
          height: "50px",
          opacity: isDragging ? 0.5 : 1,
          touchAction: draggableType ? "none" : undefined,
          width: isDragging ? 0 : "50px",
        }}
      />
    </span>
  );

  return (
    <Tooltip title={isDragging || isDraggingOther ? "" : (title as string)}>
      {imgEl}
    </Tooltip>
  );
};

export default memo(TileImg);
