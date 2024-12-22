import { DownOutlined, UpOutlined } from "@ant-design/icons";
import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { getIsLoggedIn } from "src/lib/auth";
import { SiteUrls } from "src/lib/site/urls";
import Button from "src/ui/common/button";

import PageContentComp from "../ui/common/page-content";
import Header from "./common/header";
import styles from "./page-content.module.scss";

type Props = {
  children: React.ReactNode;
  contentStyle?: React.CSSProperties;
  headerCollapsible?: boolean;
  style?: React.CSSProperties;
};

const PageContent = ({
  children,
  contentStyle,
  headerCollapsible,
  ...props
}: Props) => {
  const { t } = useTranslation();

  const [isCollapsed, setIsCollapsed] = useState(false);
  const router = useRouter();
  const isLoggedIn = getIsLoggedIn();

  const trackOffscreenGame = () => {
    router.push(SiteUrls.offscreenGame);
  };

  useEffect(() => {
    if (headerCollapsible) {
      setIsCollapsed(true);
    }
  }, [headerCollapsible]);

  return (
    <PageContentComp
      {...props}
      style={{
        paddingTop: isCollapsed ? 0 : undefined,
      }}
    >
      <div
        className={[
          styles.headerWrapper,
          isCollapsed ? styles.collapsed : "",
        ].join(" ")}
      >
        <Header />
        {headerCollapsible && (
          <div className={styles.collapse}>
            <button
              onClick={() => {
                setIsCollapsed(true);
              }}
            >
              <UpOutlined />
            </button>
          </div>
        )}
      </div>
      {isCollapsed && headerCollapsible && (
        <div className={styles.expand}>
          <button
            onClick={() => {
              setIsCollapsed(false);
            }}
          >
            <DownOutlined />
          </button>
        </div>
      )}
      <div className={styles.pageInner}>
        <main style={contentStyle}>{children}</main>
        <footer className="flex flex-col items-center justify-center gap-[24px]">
          <Button className={styles.footerButton}>
            <Link href="https://github.com/igncp/mahjong" target="_blank">
              {t("code")}
            </Link>
          </Button>
          {!!isLoggedIn && (
            <Button
              className={styles.footerButton}
              onClick={trackOffscreenGame}
              style={{ backgroundColor: "green", color: "white" }}
            >
              {t("auth.button.trackOffscreenGame")}
            </Button>
          )}
        </footer>
      </div>
    </PageContentComp>
  );
};

export default PageContent;
