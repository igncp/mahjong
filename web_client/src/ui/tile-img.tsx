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
  className?: string;
  draggableItem?: unknown;
  draggableType?: string;
  dropRef?: ConnectDropTarget;
  isDraggingOther?: boolean;
  onClick?: MouseEventHandler<HTMLSpanElement>;
  onIsDraggingChange?: (isDragging: boolean) => void;
  paddingLeft?: number;
  size?: number;
  tile?: Tile;
  tooltipFormatter?: (title?: string) => React.ReactNode;
};

const TileImg = ({
  className,
  draggableItem,
  draggableType,
  dropRef,
  isDraggingOther,
  onClick,
  onIsDraggingChange,
  paddingLeft,
  size = 50,
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
    [draggableType, draggableItem],
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

  const sizePx = `${size}px`;

  const imgEl = (
    <span
      className={className}
      onClick={onClick}
      ref={dropRef as unknown as LegacyRef<HTMLSpanElement>}
    >
      <span
        className="inline h-[1px]"
        style={{
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
          height: sizePx,
          opacity: isDragging ? 0.5 : 1,
          touchAction: draggableType ? "none" : undefined,
          width: isDragging ? 0 : sizePx,
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
