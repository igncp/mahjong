import { Button as AntdButton } from "antd";
import React from "react";

type TProps = {
  children: React.ReactNode;
  className?: string;
  disabled?: boolean;
  onClick?: () => void;
  style?: React.CSSProperties;
  type?: "dashed" | "default" | "link" | "primary";
};

const Button = (props: TProps) => <AntdButton {...props} />;

export default Button;
