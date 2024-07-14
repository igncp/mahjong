import { Space as AntdSpace } from "antd";

type Props = {
  children: React.ReactNode;
  direction?: "horizontal" | "vertical";
  fullWidth?: boolean;
  style?: React.CSSProperties;
  wrap?: boolean;
};

const Space = ({ fullWidth, style, ...props }: Props) => (
  <AntdSpace
    {...props}
    style={{ width: fullWidth ? "100%" : undefined, ...style }}
  />
);

export default Space;
