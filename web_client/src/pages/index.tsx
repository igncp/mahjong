import Head from "next/head";
import { useRouter } from "next/router";
import { useTranslation } from "react-i18next";

import { SiteUrls } from "src/lib/site/urls";
import { GameScreen, IndexScreen, OffscreenGame } from "src/screens/list";

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
          <title>{t("page.title")}</title>
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

          case paths[0] === SiteUrls.offscreenGame.split("/")[2]:
            return <OffscreenGame />;

          default:
            return <IndexScreen />;
        }
      })()}
    </>
  );
};

export default Home;
