import { MouseEventHandler, memo, useMemo } from "react";
import { ConnectDropTarget, useDrag } from "react-dnd";
import { useTranslation } from "react-i18next";

import { getTileInfo } from "src/lib/tile-info";
import { Tile } from "src/sdk/core";
import { useEffectExceptOnMount } from "src/sdk/hooks";

import Tooltip from "./common/tooltip";

type Props = {
  draggableItem?: unknown;
  draggableType?: string;
  dropRef?: ConnectDropTarget;
  isDraggingOther?: boolean;
  onClick?: MouseEventHandler<HTMLSpanElement>;
  onIsDraggingChange?: (isDragging: boolean) => void;
  paddingLeft?: number;
  tile: Tile;
  tooltipFormatter?: (title?: string) => React.ReactNode;
};

const TileImg = ({
  draggableItem,
  draggableType,
  dropRef,
  isDraggingOther,
  onClick,
  onIsDraggingChange,
  paddingLeft,
  tile,
  tooltipFormatter,
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
    <span onClick={onClick} ref={dropRef}>
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

  const tooltipContent = (() => {
    if (isDragging || isDraggingOther) return "";

    if (tooltipFormatter) return tooltipFormatter(title);

    return title || "";
  })();

  return <Tooltip title={tooltipContent}>{imgEl}</Tooltip>;
};

export default memo(TileImg);
