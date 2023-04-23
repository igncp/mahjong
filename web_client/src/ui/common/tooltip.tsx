import { Tooltip as AntdTooltip } from "antd";

type Props = {
  children: React.ReactNode;
  title: string;
};

const Card = (props: Props) => <AntdTooltip {...props} />;

export default Card;
