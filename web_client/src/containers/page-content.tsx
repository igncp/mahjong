import { DownOutlined, UpOutlined } from "@ant-design/icons";
import Link from "next/link";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

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
        <div className={styles.collapse}>
          <button
            onClick={() => {
              setIsCollapsed(true);
            }}
          >
            <UpOutlined />
          </button>
        </div>
      </div>
      {isCollapsed && (
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
        <footer>
          <Button className={styles.githubButton}>
            <Link href="https://github.com/igncp/mahjong" target="_blank">
              {t("code")}
            </Link>
          </Button>
        </footer>
      </div>
    </PageContentComp>
  );
};

export default PageContent;
