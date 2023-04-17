import Link from "next/link";
import React from "react";

import { darkGreen } from "./colors";
import Title from "./title";

interface IProps {
  children?: React.ReactNode;
  linkPath: string;
  text: string;
}

const Header = ({ linkPath, text, children }: IProps) => (
  <Title
    level={1}
    style={{
      borderBottom: "1px solid #ccc",
      boxShadow: "0 0 10px rgba(0, 0, 0, 0.1)",
      display: "flex",
      fontSize: "30px",
      margin: 0,
      padding: "30px 10px 15px",
    }}
  >
    <Link
      href={linkPath}
      style={{
        color: darkGreen,
        textDecoration: "none",
      }}
    >
      {text}
    </Link>
    {children}
  </Title>
);

export default Header;
