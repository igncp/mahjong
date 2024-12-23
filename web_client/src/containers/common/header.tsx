import Link from "next/link";
import { useRouter } from "next/router";
import React, { memo } from "react";
import { useTranslation } from "react-i18next";

import { getIsLoggedIn } from "src/lib/auth";
import { SiteUrls } from "src/lib/site/urls";
import { tokenObserver } from "src/sdk/auth";
import Button from "src/ui/common/button";
import HeaderComp from "src/ui/common/header";

import styles from "./header.module.scss";

const spacing = {
  marginLeft: "10px",
};

const Header = () => {
  const isLoggedIn = getIsLoggedIn();
  const router = useRouter();
  const { i18n, t } = useTranslation();

  const trackOffscreenGame = () => {
    router.push(SiteUrls.offscreenGame);
  };

  const isInRealGameScreen = router.asPath === SiteUrls.offscreenGame;

  return (
    <HeaderComp linkPath={SiteUrls.index} text={t("header.title", "Mahjong")}>
      <span
        style={{
          display: "inline-block",
          flex: 1,
          textAlign: "right",
        }}
      >
        {isLoggedIn && !isInRealGameScreen && (
          <Button
            className={styles.desktopButton}
            onClick={trackOffscreenGame}
            style={{ backgroundColor: "green" }}
            type="primary"
          >
            {t("auth.button.trackOffscreenGame")}
          </Button>
        )}
        <Button
          onClick={() => {
            i18n.changeLanguage(i18n.language === "en" ? "zh" : "en");
          }}
          style={spacing}
        >
          {i18n.language === "en" ? "中文" : "EN"}
        </Button>
        <Button className={styles.desktopButton} style={spacing}>
          <Link href="https://github.com/igncp/mahjong" target="_blank">
            {t("code")}
          </Link>
        </Button>
        {isLoggedIn && (
          <Button
            data-name="signout-button"
            onClick={() => {
              tokenObserver.next("");
              router.replace(SiteUrls.index);
            }}
            style={spacing}
          >
            {t("header.logout")}
          </Button>
        )}
      </span>
    </HeaderComp>
  );
};

export default memo(Header);
