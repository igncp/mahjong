import { ServiceGameSummary, TileId } from "mahjong_sdk/dist/core";
import { ModelServiceGameSummary } from "mahjong_sdk/dist/service-game-summary";
import { useCallback, useMemo } from "react";
import { useDrop } from "react-dnd";
import { Subject } from "rxjs";

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
  const canDiscardTile = getCanDiscardTile();
  const handWithoutMelds = serviceGameSummary
    ? serviceGameM.getPlayerHandWithoutMelds()
    : [];
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

  const handTilesProps = handWithoutMelds.map((_handTile, handTileIndex) => ({
    draggableItem: draggableItems[handTileIndex],
    draggableType: "handTile",
    dropRef: handDrops[handTileIndex][1],
    hasItemOver: handDrops[handTileIndex][0].isOver,
    onIsDraggingChange,
    tile: handTilesMemo[handTileIndex],
  }));

  return {
    boardDropRef,
    canDropInBoard,
    handDrops,
    handTilesProps,
  };
};
