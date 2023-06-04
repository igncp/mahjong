import styles from "./page-content.module.scss";

type Props = {
  children: React.ReactNode;
  style?: React.CSSProperties;
};

const PageContent = (props: Props) => (
  <div {...props} className={styles.content} />
);

export default PageContent;
