import Link from "next/link";
import { useRouter } from "next/router";
import { MouseEventHandler, useEffect, useState } from "react";

import Header from "src/containers/common/header";

import { HttpClient } from "../lib/http-client";
import { TAdminGetGamesResponse } from "../lib/mahjong-service";
import { SiteUrls } from "../lib/site/urls";

const DashboardAdmin = () => {
  const [page, setPage] = useState<TAdminGetGamesResponse | null>(null);
  const router = useRouter();

  useEffect(() => {
    (async () => {
      const games = await HttpClient.adminGetGames();

      setPage(games);
    })();
  }, []);

  if (!page) return null;

  const handleNewAdminGame: MouseEventHandler<HTMLAnchorElement> = async (
    ev
  ) => {
    ev.preventDefault();
    ev.stopPropagation();

    const serviceGame = await HttpClient.adminNewGame();

    router.push(SiteUrls.adminGame(serviceGame.game.id));
  };

  return (
    <main>
      <Header />
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
    </main>
  );
};

export default DashboardAdmin;
