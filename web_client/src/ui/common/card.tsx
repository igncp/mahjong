import { Card as AntdCard } from "antd";

type Props = {
  bodyStyle?: React.CSSProperties;
  children: React.ReactNode;
  className?: string;
  style?: React.CSSProperties;
  title?: React.ReactNode;
};

const Card = (props: Props) => <AntdCard {...props} />;

export default Card;
