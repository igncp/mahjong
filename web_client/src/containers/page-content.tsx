import Link from "next/link";
import { useTranslation } from "react-i18next";

import Button from "src/ui/common/button";

import PageContentComp from "../ui/common/page-content";
import Header from "./common/header";
import styles from "./page-content.module.scss";

type Props = {
  children: React.ReactNode;
  contentStyle?: React.CSSProperties;
  style?: React.CSSProperties;
};

const PageContent = ({ children, contentStyle, ...props }: Props) => {
  const { t } = useTranslation();

  return (
    <PageContentComp {...props}>
      <div className={styles.headerWrapper}>
        <Header />
      </div>
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
