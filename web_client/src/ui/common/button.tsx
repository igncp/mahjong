import { Button as AntdButton } from "antd";
import React from "react";

type TProps = {
  children: React.ReactNode;
  className?: string;
  disabled?: boolean;
  onClick?: () => void;
  style?: React.CSSProperties;
  type?: "link" | "primary" | "dashed";
};

const Button = (props: TProps) => <AntdButton {...props} />;

export default Button;
