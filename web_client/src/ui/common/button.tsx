import { Button as AntdButton } from "antd";
import React from "react";

type TProps = {
  children: React.ReactNode;
  disabled?: boolean;
  onClick?: () => void;
  type?: "link" | "primary" | "dashed";
};

const Button = (props: TProps) => <AntdButton {...props} />;

export default Button;
