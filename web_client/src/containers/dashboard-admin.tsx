import Link from "next/link";
import { useRouter } from "next/router";
import { MouseEventHandler, useEffect, useState } from "react";

import { TAdminGetGamesResponse } from "mahjong_sdk/src/core";
import { HttpClient } from "mahjong_sdk/src/http-server";

import { SiteUrls } from "../lib/site/urls";

const DashboardAdmin = () => {
  const [page, setPage] = useState<TAdminGetGamesResponse | null>(null);
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
    <ul>
      {page.map((game) => (
        <li key={game}>
          <Link href={SiteUrls.adminGame(game)}>{game}</Link>
        </li>
      ))}
      <li key="new-admin-game">
        <a href="#" onClick={handleNewAdminGame}>
          New admin game
        </a>
      </li>
    </ul>
  );
};

export default DashboardAdmin;
