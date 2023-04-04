import { useRouter } from "next/router";
import { useState } from "react";

import Header from "src/containers/common/header";
import Button from "src/ui/common/button";

import { SiteUrls } from "../lib/site/urls";

const Index = () => {
  const [playerId, setPlayerId] = useState("");
  const router = useRouter();

  return (
    <main>
      <Header />
      <ul>
        <li>
          <Button onClick={() => router.push(SiteUrls.dashboardAdmin)}>
            Admin Dashboard
          </Button>
        </li>
        <li>
          <Button
            disabled={!playerId}
            onClick={() => router.push(SiteUrls.dashboardPlayer(playerId))}
          >
            Player Dashboard
          </Button>
          <input
            onChange={(e) => setPlayerId(e.target.value)}
            type="text"
            value={playerId}
          />
        </li>
      </ul>
    </main>
  );
};

export default Index;
