import Link from "next/link";
import React from "react";

interface IProps {
  linkPath: string;
  text: string;
}

const Header = ({ linkPath, text }: IProps) => {
  return (
    <h1>
      <Link href={linkPath}>{text}</Link>
    </h1>
  );
};

export default Header;
