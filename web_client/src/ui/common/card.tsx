import { Card as AntdCard } from "antd";

type Props = {
  children: React.ReactNode;
  bodyStyle?: React.CSSProperties;
  title?: React.ReactNode;
};

const Card = (props: Props) => <AntdCard {...props} />;

export default Card;
