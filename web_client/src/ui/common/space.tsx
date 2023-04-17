import { Space as AntdSpace } from "antd";

type Props = {
  children: React.ReactNode;
  direction?: "horizontal" | "vertical";
  fullWidth?: boolean;
  style?: React.CSSProperties;
  wrap?: boolean;
};

const Space = (props: Props) => (
  <AntdSpace
    {...props}
    style={{ width: props.fullWidth ? "100%" : undefined, ...props.style }}
  />
);

export default Space;
