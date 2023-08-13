import { ServiceGameSummary } from "mahjong_sdk/dist/core";
import { ModelServiceGameSummary } from "mahjong_sdk/dist/service-game-summary";

import Text from "src/ui/common/text";
import Tooltip from "src/ui/common/tooltip";
import UserAvatar from "src/ui/common/user-avatar";
import {
  DROP_BG,
  DROP_BORDER,
  DROP_HEIGHT,
  DROP_WIDTH,
} from "src/ui/game/use-game-ui";
import TileImg from "src/ui/tile-img";

import styles from "./board.module.scss";

export type BoardPlayer = {
  id: string;
  name: string;
};

interface IProps {
  activePlayer: BoardPlayer["id"];
  canDropInBoard: boolean;
  players: [BoardPlayer, BoardPlayer, BoardPlayer, BoardPlayer];
  serviceGameM: ModelServiceGameSummary;
  serviceGameSummary: ServiceGameSummary;
}

const GameBoard = ({
  activePlayer,
  canDropInBoard,
  players,
  serviceGameM,
  serviceGameSummary,
}: IProps) => (
  <Text className={styles.wrapper}>
    <span className={styles.content}>
      {serviceGameSummary.game_summary.board.map((tileId, tileIndex, tiles) => {
        const isDiscardedTile =
          tiles.length === tileIndex + 1 &&
          typeof serviceGameSummary.game_summary.round.discarded_tile ===
            "number";
        const tile = serviceGameM.getTile(tileId);

        return (
          <span
            key={tileId}
            {...(isDiscardedTile
              ? {
                  onClick: () => {
                    serviceGameM.claimTile();
                  },
                  style: {
                    color: "blue",
                    cursor: "pointer",
                  },
                }
              : {})}
          >
            <TileImg tile={tile} />
          </span>
        );
      })}
      {players.map((player, playerIndex) => {
        const userStyle = [
          styles.userBottom,
          styles.userLeft,
          styles.userTop,
          styles.userRight,
        ][playerIndex];

        return (
          <span
            className={[styles.userWrapper, userStyle]
              .concat(activePlayer === player.id ? [styles.userActive] : [])
              .join(" ")}
            key={player.id}
          >
            <Tooltip title={player.name}>
              <span>
                <UserAvatar />
              </span>
            </Tooltip>
          </span>
        );
      })}
      {canDropInBoard && (
        <span
          style={{
            background: DROP_BG,
            border: DROP_BORDER,
            height: DROP_HEIGHT,
            marginLeft: "10px",
            opacity: 0.8,
            width: DROP_WIDTH,
          }}
        />
      )}
    </span>
  </Text>
);

export default GameBoard;
