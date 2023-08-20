import { QuestionCircleOutlined, ThunderboltOutlined } from "@ant-design/icons";
import { ServiceGameSummary } from "mahjong_sdk/dist/core";
import { ModelServiceGameSummary } from "mahjong_sdk/dist/service-game-summary";
import { memo, useState } from "react";
import { useTranslation } from "react-i18next";

import Modal from "src/ui/common/modal";
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
  dealerPlayer: BoardPlayer["id"];
  players: [BoardPlayer, BoardPlayer, BoardPlayer, BoardPlayer];
  serviceGameM: ModelServiceGameSummary;
  serviceGameSummary: ServiceGameSummary;
}

const DealerIcon = ThunderboltOutlined;

const GameBoard = ({
  activePlayer,
  canDropInBoard,
  dealerPlayer,
  players,
  serviceGameM,
  serviceGameSummary,
}: IProps) => {
  const [displayHelpModal, setDisplayHelpModal] = useState(false);
  const { t } = useTranslation();

  return (
    <Text className={styles.wrapper}>
      <span className={styles.content}>
        {serviceGameSummary.game_summary.board.map(
          (tileId, tileIndex, tiles) => {
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
                      className: styles.discardedTile,
                      onClick: () => {
                        serviceGameM.claimTile();
                      },
                    }
                  : {})}
              >
                <TileImg tile={tile} />
              </span>
            );
          }
        )}
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
                <span className={styles.userItem}>
                  <UserAvatar />
                  {dealerPlayer === player.id && (
                    <span
                      className={[styles.dealerIcon, styles.boardIcon].join(
                        " "
                      )}
                    >
                      <DealerIcon />
                    </span>
                  )}
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
        <span
          className={styles.helpModalTrigger}
          onClick={() => setDisplayHelpModal(true)}
        >
          <QuestionCircleOutlined />
        </span>
        <Modal
          footer={null}
          onCancel={() => setDisplayHelpModal(false)}
          open={displayHelpModal}
        >
          <p>
            {t(
              "board.help.intro",
              "The highlighted user is the one who should play in the current turn"
            )}
          </p>
          <p>{t("symbols", "The meaning of the symbols:")}</p>
          <ul>
            <li>
              <span className={[styles.dealerIcon, styles.helpIcon].join(" ")}>
                <DealerIcon />
              </span>{" "}
              {t("board.help.dealer", "Dealer player")}
            </li>
          </ul>
        </Modal>
      </span>
    </Text>
  );
};

export default memo(GameBoard);
