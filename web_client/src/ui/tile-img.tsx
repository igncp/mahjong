import type { Tile } from "bindings/Tile";
import type { LegacyRef, MouseEventHandler } from "react";
import { memo, useMemo } from "react";
import type { ConnectDropTarget } from "react-dnd";
import { useDrag } from "react-dnd";
import { useTranslation } from "react-i18next";

import { getTileInfo } from "src/lib/tile-info";
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
  tile?: Tile;
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

  const { language } = i18n;

  const [image, title] =
    useMemo(() => {
      language;

      if (!tile) return [];

      return getTileInfo(tile, i18n);
    }, [language, tile, i18n]) || [];

  useEffectExceptOnMount(() => {
    onIsDraggingChange?.(isDragging);
  }, [isDragging]);

  if (!image) {
    return null;
  }

  const imgEl = (
    <span
      onClick={onClick}
      ref={dropRef as unknown as LegacyRef<HTMLSpanElement>}
    >
      <span
        style={{
          display: "inline-block",
          height: "1px",
          transition: "width 0.25s",
          width: `${paddingLeft || 0}px`,
        }}
      />
      <img
        ref={
          draggableType
            ? (dragRef as unknown as LegacyRef<HTMLImageElement>)
            : undefined
        }
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
