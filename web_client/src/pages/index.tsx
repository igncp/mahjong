import Head from "next/head";
import { useRouter } from "next/router";
import { useTranslation } from "react-i18next";

import { GameScreen, IndexScreen } from "src/screens/list";

const Home = () => {
  const { asPath } = useRouter();
  const routes = asPath.split("#");
  const mainPath = routes[1] || "";
  const paths = mainPath.split("/").filter((path) => path);

  // This hook has issues in the page level
  const { t } =
    // eslint-disable-next-line react-hooks/rules-of-hooks
    typeof window === "undefined" ? { t: () => null } : useTranslation();

  return (
    <>
      {typeof window !== "undefined" && (
        <Head>
          <title>{t("page.title", "Mahjong Web Client")}</title>
        </Head>
      )}
      {(() => {
        switch (true) {
          case paths[0] === "game":
            return (
              <GameScreen
                gameId={paths[1] as string}
                gameType={paths[2] || ""}
                userId={paths[3] || ""}
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
