import Link from "next/link";
import React from "react";

import styles from "./header.module.scss";
import Title from "./title";

interface IProps {
  children?: React.ReactNode;
  linkPath: string;
  text: string;
}

const Header = ({ linkPath, text, children }: IProps) => (
  <Title className={styles.wrapper} level={1}>
    <Link className={styles.mainLink} href={linkPath}>
      {text}
    </Link>
    {children}
  </Title>
);

export default Header;
