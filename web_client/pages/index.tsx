import Head from "next/head";
import { useRouter } from "next/router";

import { GameScreen, IndexScreen } from "src/screens/list";

const Home = () => {
  const { asPath } = useRouter();
  console.log("debug: index.tsx: asPath", asPath);
  const routes = asPath.split("#");
  const mainPath = routes[1] || "";
  const paths = mainPath.split("/");
  console.log("debug: index.tsx: paths", paths);

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
