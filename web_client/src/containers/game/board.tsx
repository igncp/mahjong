import {
  CaretRightFilled,
  QuestionCircleOutlined,
  SettingFilled,
  ThunderboltOutlined,
} from "@ant-design/icons";
import type { ServiceGameSummary } from "bindings/ServiceGameSummary";
import { memo, useState } from "react";
import { useTranslation } from "react-i18next";

import type { ModelServiceGameSummary } from "src/sdk/service-game-summary";
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
import Settings from "./settings";

export type BoardPlayer = {
  id: string;
  name: string;
};

interface IProps {
  activePlayer: BoardPlayer["id"];
  canDropInBoard: boolean;
  dealerPlayer: BoardPlayer["id"];
  isMobile: boolean;
  players: [BoardPlayer, BoardPlayer, BoardPlayer, BoardPlayer];
  serviceGameM: ModelServiceGameSummary;
  serviceGameSummary: ServiceGameSummary;
}

const DealerIcon = ThunderboltOutlined;
const MeldIcon = CaretRightFilled;

const GameBoard = ({
  activePlayer,
  canDropInBoard,
  dealerPlayer,
  isMobile,
  players,
  serviceGameM,
  serviceGameSummary,
}: IProps) => {
  const [displayHelpModal, setDisplayHelpModal] = useState(false);
  const [displaySettingsModal, setDisplaySettingsModal] = useState(false);
  const { t } = useTranslation();

  const bannerDisplay = `${t("game.board")} (${
    serviceGameSummary.game_summary.board.length
  } / 92)`;

  return (
    <div className={[styles.wrapper, isMobile ? styles.mobile : ""].join(" ")}>
      <div className={styles.banner}>
        <Text>{bannerDisplay}</Text>
      </div>
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

          const isCurrentPlayer =
            player.id === serviceGameSummary.game_summary.player_id;

          const playerVisibleMelds = serviceGameSummary.game_summary.hand?.list
            ? new Set(
                isCurrentPlayer
                  ? serviceGameSummary.game_summary.hand.list
                      .filter((h) => !h.concealed)
                      .map((h) => h.set_id)
                      .filter(Boolean)
                  : serviceGameSummary.game_summary.other_hands[
                      player.id
                    ]?.visible.list
                      .map((handTile) => handTile.set_id)
                      .filter(Boolean)
              ).size
            : 0;

          const tooltip = (
            <>
              <span>{player.name}</span>
              <br />
              <span>
                {t("game.points", {
                  count: serviceGameSummary.game_summary.score[player.id],
                })}
              </span>
              <br />
              <span>
                {t("game.bonusTiles", {
                  count:
                    serviceGameSummary.game_summary.bonus_tiles[player.id]
                      ?.length || 0,
                })}
              </span>
            </>
          );

          return (
            <span
              className={[styles.userWrapper, userStyle]
                .concat(activePlayer === player.id ? [styles.userActive] : [])
                .join(" ")}
              key={player.id}
            >
              <Tooltip title={tooltip}>
                <span className={styles.userItem}>
                  <UserAvatar />
                  <span className={styles.userIcons}>
                    {dealerPlayer === player.id && (
                      <span
                        className={[styles.dealerIcon, styles.boardIcon].join(
                          " "
                        )}
                      >
                        <DealerIcon rev="" />
                      </span>
                    )}
                    {Array.from({ length: playerVisibleMelds }).map(
                      (_, index) => (
                        <span
                          className={[styles.meldIcon, styles.boardIcon].join(
                            " "
                          )}
                          key={index}
                        >
                          <MeldIcon rev="" />
                        </span>
                      )
                    )}
                  </span>
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
          <QuestionCircleOutlined rev="" />
        </span>
        <span
          className={styles.settingsModalTrigger}
          onClick={() => setDisplaySettingsModal(true)}
        >
          <SettingFilled rev="" />
        </span>
        <Modal
          footer={null}
          onCancel={() => setDisplayHelpModal(false)}
          open={displayHelpModal}
        >
          <p>{t("board.help.intro")}</p>
          <p>{t("symbols")}</p>
          <ul>
            <li>
              <span className={[styles.dealerIcon, styles.helpIcon].join(" ")}>
                <DealerIcon rev="" />
              </span>{" "}
              {t("board.help.dealer")}
            </li>
            <li>
              <span className={[styles.meldIcon, styles.helpIcon].join(" ")}>
                <MeldIcon rev="" />
              </span>{" "}
              {t("board.help.meld", "Visible meld")}
            </li>
          </ul>
        </Modal>
        <Modal
          footer={null}
          onCancel={() => setDisplaySettingsModal(false)}
          open={displaySettingsModal}
        >
          <Settings
            serviceGameM={serviceGameM}
            serviceGameSummary={serviceGameSummary}
          />
        </Modal>
      </span>
    </div>
  );
};

export default memo(GameBoard);
