import Link from "next/link";
import { useEffect, useState } from "react";

import Header from "src/containers/common/header";

import { HttpClient } from "../lib/http-client";
import { TUserGetGamesResponse } from "../lib/mahjong-service";
import { SiteUrls } from "../lib/site/urls";

type TProps = {
  userId: string;
};

const DashboardUser = ({ userId }: TProps) => {
  const [page, setPage] = useState<TUserGetGamesResponse | null>(null);

  useEffect(() => {
    (async () => {
      const games = await HttpClient.userGetGames({
        player_id: userId,
      });

      setPage(games);
    })();
  }, []);

  if (!page) return null;

  return (
    <main>
      <Header />
      <h1>Player games:</h1>
      <ul>
        {page.map((game) => (
          <li key={game}>
            <Link href={SiteUrls.playerGame(game, userId)}>{game}</Link>
          </li>
        ))}
      </ul>
    </main>
  );
};

export default DashboardUser;
