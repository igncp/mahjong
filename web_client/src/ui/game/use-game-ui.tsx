import { ServiceGameSummary, TileId } from "mahjong_sdk/dist/core";
import { ModelServiceGameSummary } from "mahjong_sdk/dist/service-game-summary";
import { MouseEventHandler, useCallback, useMemo } from "react";
import { useDrop } from "react-dnd";
import { useTranslation } from "react-i18next";
import { Subject } from "rxjs";
import { getIsSameTile } from "ts_sdk/dist/tile-content";

export const DROP_BG = "#e7e7e7";
export const DROP_BORDER = "2px solid #333";
export const DROP_HEIGHT = "40px";
export const DROP_WIDTH = "20px";

enum DropType {
  HAND_TILE = "handTile",
}

type Opts = {
  getCanDiscardTile: () => boolean;
  serviceGameM: ModelServiceGameSummary;
  serviceGameSummary: ServiceGameSummary | null;
};

export const useGameUI = ({
  getCanDiscardTile,
  serviceGameM,
  serviceGameSummary,
}: Opts) => {
  const { t } = useTranslation();
  const canDiscardTile = getCanDiscardTile();
  const handWithoutMelds = serviceGameSummary
    ? serviceGameM.getPlayerHandWithoutMelds()
    : [];
  const handMelds = (serviceGameSummary?.game_summary.hand || []).filter(
    (t) => !!t.set_id
  );
  const [{ canDropInBoard }, boardDropRef] = useDrop(
    {
      accept: DropType.HAND_TILE,
      canDrop: () => canDiscardTile,
      collect: (monitor) => ({
        canDropInBoard: !!monitor.canDrop(),
      }),
      drop: ({ tileId }: { tileId: TileId }) => {
        if (canDiscardTile) {
          serviceGameM.discardTile(tileId);
        }
      },
    },
    [canDiscardTile]
  );

  const handDrops = Array.from({ length: 14 }).map((_, index) =>
    useDrop(
      {
        accept: DropType.HAND_TILE,
        collect: (monitor) => ({
          isOver: !!monitor.isOver(),
        }),
        drop: ({ tileId }: { tileId: TileId }) => {
          const handWithoutMelds = serviceGameM.getPlayerHandWithoutMelds();
          const handIds = handWithoutMelds.map((t) => t.id);
          const tileIndex = handIds.findIndex((t) => t === tileId);

          if (tileIndex === -1) {
            return;
          }

          handIds.splice(tileIndex, 1);
          handIds.splice(index, 0, tileId);

          serviceGameM.sortHands(handIds);
        },
      },
      [serviceGameM]
    )
  );

  const handHash = handWithoutMelds.map((t) => t.id).join(",");
  const draggableItems = useMemo(
    () =>
      handWithoutMelds.map((handTile) => ({
        tileId: handTile.id,
      })),
    [handHash]
  );
  const isDragging$ = useMemo(() => new Subject<boolean>(), []);
  const onIsDraggingChange = useCallback(
    (isDragging: boolean) => {
      isDragging$.next(isDragging);
    },
    [isDragging$]
  );

  const handTilesMemo = useMemo(
    () => handWithoutMelds.map((handTile) => serviceGameM.getTile(handTile.id)),
    [handHash]
  );

  const handClickProps = useMemo(
    () =>
      handWithoutMelds.map((handTile) => {
        const handler: MouseEventHandler<HTMLSpanElement> = (e) => {
          const canDiscardTile = getCanDiscardTile();

          if (e.detail === 2 && canDiscardTile) {
            serviceGameM.discardTile(handTile.id);
          }
        };

        return handler;
      }),
    [handHash]
  );

  const board = serviceGameSummary?.game_summary.board;
  const visibleMelds = Object.values(
    serviceGameSummary?.game_summary.other_hands || {}
  )
    .map((otherHand) => otherHand.visible)
    .flat();

  const tooltipFormatters = useMemo(
    () =>
      handWithoutMelds.map((handTile) =>
        // eslint-disable-next-line react/display-name
        (title?: string) => {
          const tile = serviceGameM.getTile(handTile.id);
          const sameTilesInBoard = board?.filter((t) => {
            const boardTile = serviceGameM.getTile(t);
            return getIsSameTile(boardTile, tile);
          }).length;
          const sameTilesInMelds = visibleMelds
            .concat(handMelds)
            .filter((otherHandTile) => {
              const otherTile = serviceGameM.getTile(otherHandTile.id);
              return getIsSameTile(otherTile, tile);
            }).length;

          return (
            <>
              {title}
              <br />
              {t("tilesInBoard", "In board: {{count}}", {
                count: sameTilesInBoard,
              })}
              <br />
              {t("tilesInMelds", "In other melds: {{count}}", {
                count: sameTilesInMelds,
              })}
            </>
          );
        }
      ),
    [handHash, board, t, visibleMelds, handMelds]
  );

  const handTilesProps = handWithoutMelds.map((_handTile, handTileIndex) => ({
    draggableItem: draggableItems[handTileIndex],
    draggableType: "handTile",
    dropRef: handDrops[handTileIndex][1],
    hasItemOver: handDrops[handTileIndex][0].isOver,
    onClick: handClickProps[handTileIndex],
    onIsDraggingChange,
    tile: handTilesMemo[handTileIndex],
    tooltipFormatter: tooltipFormatters[handTileIndex],
  }));

  return {
    boardDropRef,
    canDropInBoard,
    handDrops,
    handTilesProps,
  };
};
