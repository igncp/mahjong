import Link from "next/link";
import { useRouter } from "next/router";
import { MouseEventHandler, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { TAdminGetGamesResponse } from "mahjong_sdk/src/core";
import { HttpClient } from "mahjong_sdk/src/http-server";
import List, { ListItem } from "src/ui/common/list";

import { SiteUrls } from "../lib/site/urls";
import PageContent from "./page-content";

const DashboardAdmin = () => {
  const [page, setPage] = useState<TAdminGetGamesResponse | null>(null);
  const { t } = useTranslation();
  const router = useRouter();

  useEffect(() => {
    HttpClient.adminGetGames().subscribe({
      next: (games) => {
        setPage(games);
      },
    });
  }, []);

  if (!page) return null;

  const handleNewAdminGame: MouseEventHandler<HTMLAnchorElement> = (ev) => {
    ev.preventDefault();
    ev.stopPropagation();

    HttpClient.adminNewGame().subscribe({
      next: (serviceGame) => {
        router.push(SiteUrls.adminGame(serviceGame.game.id));
      },
    });
  };

  return (
    <PageContent>
      <List
        bordered
        dataSource={page}
        renderItem={(game) => (
          <ListItem>
            <Link href={SiteUrls.adminGame(game)}>{game}</Link>
          </ListItem>
        )}
      />
      <ul>
        <li key="new-admin-game">
          <a href="#" onClick={handleNewAdminGame}>
            {t("dashboard.newAdminGame", "New admin game")}
          </a>
        </li>
      </ul>
    </PageContent>
  );
};

export default DashboardAdmin;
