import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { HttpClient } from "../lib/http-client";
import { TAdminGetGamesResponse } from "../lib/mahjong-service";
import { SiteUrls } from "../lib/site/urls";
import Header from "src/containers/common/header";

const Index = () => {
  const [page, setPage] = useState<TAdminGetGamesResponse | null>(null);
  const router = useRouter();

  useEffect(() => {
    (async () => {
      const httpClient = HttpClient.singleton();
      const games = await httpClient.adminGetGames();

      setPage(games);
    })();
  }, []);

  if (!page) return null;

  const handleNewAdminGame = async (ev: any) => {
    ev.preventDefault();
    ev.stopPropagation();

    const httpClient = HttpClient.singleton();
    const serviceGame = await httpClient.adminNewGame();

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
          <a onClick={handleNewAdminGame} href="#">
            New admin game
          </a>
        </li>
      </ul>
    </main>
  );
};

export default Index;
