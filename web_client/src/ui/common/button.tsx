import React from "react";

type TProps = {
  children: React.ReactNode;
  onClick?: () => void;
};

const Button = ({ children, onClick }: TProps) => {
  return (
    <button onClick={onClick} style={{ cursor: "pointer" }}>
      {children}
    </button>
  );
};

export default Button;
