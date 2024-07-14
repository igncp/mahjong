import type { ServiceGameSummary } from "bindings/ServiceGameSummary";
import type { MouseEventHandler } from "react";
import { useCallback, useMemo } from "react";
import { useDrop } from "react-dnd";
import { useTranslation } from "react-i18next";
import { Subject } from "rxjs";

import type { TileId } from "src/sdk/core";
import type { ModelServiceGameSummary } from "src/sdk/service-game-summary";
import { getIsSameTile } from "src/sdk/tile-content";

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
  serviceGameSummary: null | ServiceGameSummary;
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
    : null;

  const handMelds = (serviceGameSummary?.game_summary.hand.list || []).filter(
    (h) => !!h.set_id
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
    // eslint-disable-next-line react-hooks/rules-of-hooks
    useDrop(
      {
        accept: DropType.HAND_TILE,
        collect: (monitor) => ({
          isOver: !!monitor.isOver(),
        }),
        drop: ({ tileId }: { tileId: TileId }) => {
          const handWithoutMeldsNew = serviceGameM.getPlayerHandWithoutMelds();
          const handIds = handWithoutMeldsNew.list.map((h) => h.id);
          const tileIndex = handIds.findIndex((h) => h === tileId);

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

  const handHash = handWithoutMelds?.list.map((h) => h.id).join(",");

  const draggableItems = useMemo(
    () =>
      handWithoutMelds?.list.map((handTile) => ({
        tileId: handTile.id,
      })),
    // eslint-disable-next-line react-hooks/exhaustive-deps
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
    () =>
      handWithoutMelds?.list.map((handTile) =>
        serviceGameM.getTile(handTile.id)
      ),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [handHash]
  );

  const handClickProps = useMemo(
    () =>
      handWithoutMelds?.list.map((handTile) => {
        const handler: MouseEventHandler<HTMLSpanElement> = (e) => {
          const canDiscardTileNew = getCanDiscardTile();

          if (e.detail === 2 && canDiscardTileNew) {
            serviceGameM.discardTile(handTile.id);
          }
        };

        return handler;
      }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
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
      handWithoutMelds?.list.map((handTile) =>
        // eslint-disable-next-line react/display-name
        (title?: string) => {
          const tile = serviceGameM.getTile(handTile.id);

          const sameTilesInBoard = board?.filter((ti) => {
            const boardTile = serviceGameM.getTile(ti);

            return getIsSameTile(boardTile, tile);
          }).length;

          const sameTilesInMelds = visibleMelds
            .map((h) => h.list)
            .flat()
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
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [handHash, board, t, visibleMelds, handMelds]
  );

  const handTilesProps = handWithoutMelds?.list.map(
    (_handTile, handTileIndex) => ({
      draggableItem: draggableItems?.[handTileIndex],
      draggableType: "handTile",
      dropRef: handDrops[handTileIndex][1],
      hasItemOver: handDrops[handTileIndex][0].isOver,
      onClick: handClickProps?.[handTileIndex],
      onIsDraggingChange,
      tile: handTilesMemo?.[handTileIndex],
      tooltipFormatter: tooltipFormatters?.[handTileIndex],
    })
  );

  return {
    boardDropRef,
    canDropInBoard,
    handDrops,
    handTilesProps,
  };
};
