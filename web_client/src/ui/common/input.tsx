import { Input as AntdInput } from "antd";
import type { ChangeEvent } from "react";
import React from "react";

type TProps = {
  disabled?: boolean;
  onChange?: (value: ChangeEvent<HTMLInputElement>) => void;
  onPressEnter?: () => void;
  placeholder?: string;
  type?: "password" | "text";
  value?: string;
};

const Input = (props: TProps) => <AntdInput {...props} />;

export default Input;
