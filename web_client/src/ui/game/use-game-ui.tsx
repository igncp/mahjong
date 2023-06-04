import { useCallback, useEffect, useMemo, useState } from "react";
import { useDrop } from "react-dnd";
import { Subject } from "rxjs";

import { ServiceGameSummary, TileId } from "mahjong_sdk/src/core";
import { ModelServiceGameSummary } from "mahjong_sdk/src/service-game-summary";

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
  const LeftDrops = useMemo(
    () =>
      handWithoutMelds.map((_handTile, handTileIndex) => {
        const LeftDrop = () => {
          const [{ isOver }] = handDrops[handTileIndex];
          const [isDragging, setIsDragging] = useState(false);

          const background = (() => {
            switch (true) {
              case isOver:
                return "#ccc";
              case isDragging:
                return DROP_BG;
              default:
                return "white";
            }
          })();

          useEffect(() => {
            const sub = isDragging$.subscribe((newIsDragging) => {
              setTimeout(() => {
                setIsDragging(newIsDragging);
              });
            });
            return () => sub.unsubscribe();
          }, []);

          return (
            <span
              className="hand-drop"
              ref={handDrops[handTileIndex][1]}
              style={{
                background,
                border: isDragging ? DROP_BORDER : "none",
                display: "inline-block",
                height: DROP_HEIGHT,
                width: isDragging ? "20px" : "0px",
              }}
            />
          );
        };

        return LeftDrop;
      }),
    [handHash]
  );
  const handTilesMemo = useMemo(
    () => handWithoutMelds.map((handTile) => serviceGameM.getTile(handTile.id)),
    [handHash]
  );

  const handTilesProps = handWithoutMelds.map((_handTile, handTileIndex) => ({
    draggableItem: draggableItems[handTileIndex],
    draggableType: "handTile",
    LeftDrop: LeftDrops[handTileIndex],
    onIsDraggingChange,
    tile: handTilesMemo[handTileIndex],
  }));

  return {
    boardDropRef,
    canDropInBoard,
    handTilesProps,
  };
};
