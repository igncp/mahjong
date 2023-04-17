import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";

import { tokenObserver } from "src/lib/auth";
import Button from "src/ui/common/button";
import List, { ListItem } from "src/ui/common/list";
import PageContent from "src/ui/common/page-content";
import Title from "src/ui/common/title";

import { HttpClient } from "../lib/http-client";
import { TUserGetGamesResponse } from "../lib/mahjong-service";
import { SiteUrls } from "../lib/site/urls";

type TProps = {
  userId: string;
};

const DashboardUser = ({ userId }: TProps) => {
  const [gamesIds, setPage] = useState<TUserGetGamesResponse | null>(null);
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

  if (!gamesIds) return null;

  return (
    <PageContent>
      <Title level={2}>Player games:</Title>
      <List
        bordered
        dataSource={gamesIds}
        renderItem={(gameId) => (
          <ListItem>
            <Link href={SiteUrls.playerGame(gameId, userId)}>{gameId}</Link>
          </ListItem>
        )}
      />
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
    </PageContent>
  );
};

export default DashboardUser;
