import Link from "next/link";

import Button from "src/ui/common/button";

import PageContentComp from "../ui/common/page-content";
import Header from "./common/header";
import styles from "./page-content.module.scss";

type Props = {
  children: React.ReactNode;
  style?: React.CSSProperties;
  contentStyle?: React.CSSProperties;
};

const PageContent = ({ children, contentStyle, ...props }: Props) => (
  <PageContentComp {...props}>
    <div className={styles.headerWrapper}>
      <Header />
    </div>
    <div className={styles.pageInner}>
      <main style={contentStyle}>{children}</main>
      <footer>
        <Button className={styles.githubButton}>
          <Link href="https://github.com/igncp/mahjong" target="_blank">
            GitHub
          </Link>
        </Button>
      </footer>
    </div>
  </PageContentComp>
);

export default PageContent;
