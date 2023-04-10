import Link from "next/link";
import React from "react";

interface IProps {
  children?: React.ReactNode;
  linkPath: string;
  text: string;
}

const Header = ({ linkPath, text, children }: IProps) => (
  <h1 style={{ display: "flex" }}>
    <Link href={linkPath}>{text}</Link>
    {children}
  </h1>
);

export default Header;
