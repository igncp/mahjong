import {
  InfoCircleOutlined,
  QuestionCircleOutlined,
  SettingFilled,
  ThunderboltOutlined,
} from "@ant-design/icons";
import type { ServiceGameSummary } from "bindings/ServiceGameSummary";
import type { Wind } from "bindings/Wind";
import { useState } from "react";
import { useTranslation } from "react-i18next";

import type { PlayingExtrasParsed } from "src/sdk/pkg-wrapper";

import type { TileId } from "src/sdk/core";
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
  activePlayer: BoardPlayer["id"] | undefined;
  canDropInBoard: boolean;
  dealerPlayer: BoardPlayer["id"] | undefined;
  getMeldType: (tiles: TileId[]) => string;
  isMobile: boolean;
  players: [BoardPlayer, BoardPlayer, BoardPlayer, BoardPlayer];
  playersVisibleMelds: PlayingExtrasParsed["players_visible_melds"] | undefined;
  playersWinds: PlayingExtrasParsed["players_winds"] | undefined;
  serviceGameM: ModelServiceGameSummary;
  serviceGameSummary: ServiceGameSummary;
  windToText: Record<Wind, string>;
}

const DealerIcon = ThunderboltOutlined;

const GameBoard = ({
  activePlayer,
  canDropInBoard,
  dealerPlayer,
  getMeldType,
  isMobile,
  players,
  playersVisibleMelds,
  playersWinds,
  serviceGameM,
  serviceGameSummary,
  windToText,
}: IProps) => {
  const [displayHelpModal, setDisplayHelpModal] = useState(false);
  const [displaySettingsModal, setDisplaySettingsModal] = useState(false);
  const { t } = useTranslation();

  const playerIndex = players.findIndex(
    (player) => player.id === serviceGameSummary.game_summary.player_id,
  );

  const sortedPlayers = [
    ...players.slice(playerIndex),
    ...players.slice(0, playerIndex),
  ];

  const bannerDisplay = `${t("game.board")}: ${t(
    "game.boardRemaining",
    "{{existing}}, {{count}} remaining",
    {
      count: serviceGameSummary.game_summary.draw_wall_count,
      existing: serviceGameSummary.game_summary.board.length,
    },
  )}`;

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
          },
        )}
        {sortedPlayers.map((player, idx) => {
          const userStyle = [
            styles.userBottom,
            styles.userLeft,
            styles.userTop,
            styles.userRight,
          ][idx];

          const playerWind = playersWinds?.get(player.id);

          const visibleMelds = playersVisibleMelds?.get(player.id);

          const tilesColumn = (visibleMelds || []).map((visibleMeld) => {
            const { set_id: setId, tiles } = visibleMeld;
            const meldType = getMeldType(tiles);

            const tooltipTitle = (
              <span>
                <b>
                  {t("game.visibleMeld", {
                    player: player.name,
                  })}
                </b>
                <br />
                {meldType}
              </span>
            );

            return (
              <div
                className="flex flex-row items-center justify-center gap-[1px]"
                key={setId}
              >
                {tiles.map((tileId) => {
                  const tile = serviceGameM.getTile(tileId);

                  return (
                    <span key={tileId}>
                      <TileImg size={15} tile={tile} />
                    </span>
                  );
                })}
                <Tooltip title={tooltipTitle}>
                  <InfoCircleOutlined size={4} />
                </Tooltip>
              </div>
            );
          });

          const avatarTooltip = (
            <>
              <span>
                {player.name}
                {playerWind ? ` | ${windToText[playerWind]}` : ""}
              </span>
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
              <span className={styles.userContentWrap}>
                <div className={styles.userContent}>
                  <Tooltip title={avatarTooltip}>
                    <div className={styles.userItem}>
                      <UserAvatar />
                    </div>
                  </Tooltip>
                  <div className={styles.userIcons}>
                    {dealerPlayer === player.id && (
                      <div className={[styles.dealerIcon].join(" ")}>
                        <DealerIcon rev="" />
                      </div>
                    )}
                  </div>
                  {!!visibleMelds?.length && (
                    <div className={styles.tilesColumn}>{tilesColumn}</div>
                  )}
                </div>
              </span>
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

export default GameBoard;
