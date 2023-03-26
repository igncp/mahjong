import dynamic from "next/dynamic";
import Head from "next/head";
import { useRouter } from "next/router";

const IndexScreen = dynamic(() => import("../src/screens/index"), {
  ssr: false,
});
const GameScreen = dynamic(() => import("../src/screens/game"), { ssr: false });

const Home = () => {
  const { asPath } = useRouter();
  const routes = asPath.split("#");
  const paths = (routes[1] || "").split("/");

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
