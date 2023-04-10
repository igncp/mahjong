import React from "react";

type TProps = {
  children: React.ReactNode;
  onClick?: () => void;
} & React.ButtonHTMLAttributes<HTMLButtonElement>;

const Button = (props: TProps) => (
  <button style={props.disabled ? {} : { cursor: "pointer" }} {...props} />
);

export default Button;
