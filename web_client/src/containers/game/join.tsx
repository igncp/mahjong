import message from "antd/es/message";
import { useRouter } from "next/router";
import { useTranslation } from "react-i18next";
import { first } from "rxjs/operators";

import { getIsLoggedIn } from "src/lib/auth";
import { SiteUrls } from "src/lib/site/urls";
import { HttpClient } from "src/sdk/http-client";
import Button from "src/ui/common/button";
import Modal from "src/ui/common/modal";

import PageContent from "../page-content";

export type IGameJoinScreenProps = {
  gameId: string;
};

const GameJoinScreen = ({ gameId }: IGameJoinScreenProps) => {
  const { t } = useTranslation();
  const isLoggedIn = getIsLoggedIn();
  const [messageApi, contextHolder] = message.useMessage();
  const router = useRouter();

  return (
    <PageContent>
      <Modal
        footer={[
          <Button
            disabled={!isLoggedIn}
            key="yes"
            onClick={() => {
              HttpClient.userJoinGame(gameId)
                .pipe(first())
                .subscribe({
                  error: (error) => {
                    console.error(error);

                    messageApi.error(
                      t("join.errorJoiningGame", "Error joining game"),
                    );
                  },
                  next: (userId) => {
                    router.replace(SiteUrls.playerGame(gameId, userId));
                  },
                });
            }}
            type="primary"
          >
            {t("dashboard.yes", "Yes")}
          </Button>,
        ]}
        onCancel={() => {
          window.location.href = "/";
        }}
        open
      >
        <h1>{t("join.joinGame", "Do you want to join this game:")}</h1>
        <div>
          <b>{gameId}</b>
        </div>
        {!isLoggedIn && (
          <div>
            {t("join.loginToJoin", "You need to login to join the game.")}
          </div>
        )}
      </Modal>
      {contextHolder}
    </PageContent>
  );
};

export default GameJoinScreen;
