import Link from "next/link";
import React from "react";

import styles from "./header.module.scss";
import Title from "./title";

interface IProps {
  children?: React.ReactNode;
  linkPath: string;
  text: string;
}

const Header = ({ children, linkPath, text }: IProps) => (
  <Title className={styles.wrapper} level={1}>
    <span className={styles.inner}>
      <Link className={styles.mainLink} href={linkPath}>
        {text}
      </Link>
      {children}
    </span>
  </Title>
);

export default Header;
