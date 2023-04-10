import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";

import { tokenObserver } from "src/lib/auth";
import Button from "src/ui/common/button";

import { HttpClient } from "../lib/http-client";
import { TUserGetGamesResponse } from "../lib/mahjong-service";
import { SiteUrls } from "../lib/site/urls";

type TProps = {
  userId: string;
};

const DashboardUser = ({ userId }: TProps) => {
  const [page, setPage] = useState<TUserGetGamesResponse | null>(null);
  const router = useRouter();

  useEffect(() => {
    (async () => {
      const games = await HttpClient.userGetGames({
        player_id: userId,
      }).catch(() => {
        tokenObserver.next(null);
        return null;
      });

      if (!games) return;

      setPage(games);
    })();
  }, []);

  if (!page) return null;

  return (
    <>
      <h1>Player games:</h1>
      <ul>
        {page.map((game) => (
          <li key={game}>
            <Link href={SiteUrls.playerGame(game, userId)}>{game}</Link>
          </li>
        ))}
        <li>
          <Button
            onClick={async () => {
              const game = await HttpClient.userCreateGame({
                player_id: userId,
              });

              router.push(SiteUrls.playerGame(game.game_summary.id, userId));
            }}
          >
            Create game
          </Button>
        </li>
      </ul>
    </>
  );
};

export default DashboardUser;
