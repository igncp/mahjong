import Head from "next/head";
import { useRouter } from "next/router";

import { GameScreen, IndexScreen } from "src/screens/list";

const Home = () => {
  const { asPath } = useRouter();
  const routes = asPath.split("#");
  const mainPath = routes[1] || "";
  const paths = mainPath.split("/");

  return (
    <>
      <Head>
        <title>Mahjong Web Client</title>
      </Head>
      {(() => {
        switch (true) {
          case paths[1] === "game":
            return (
              <GameScreen
                gameId={paths[2] as string}
                gameType={paths[3] || ""}
                userId={paths[4] || ""}
              />
            );

          default:
            return <IndexScreen />;
        }
      })()}
    </>
  );
};

export default Home;
