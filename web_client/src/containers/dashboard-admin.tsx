import Link from "next/link";
import { useRouter } from "next/router";
import { MouseEventHandler, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { TAdminGetGamesResponse } from "src/sdk/core";
import { HttpClient } from "src/sdk/http-client";
import List, { ListItem } from "src/ui/common/list";

import { SiteUrls } from "../lib/site/urls";
import PageContent from "./page-content";

const DashboardAdmin = () => {
  const [page, setPage] = useState<null | TAdminGetGamesResponse>(null);
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
            <Link href={SiteUrls.adminGame(game.id)}>{game.id}</Link>
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
