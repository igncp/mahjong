import { Typography } from "antd";

const { Title: AntdTitle } = Typography;

type Props = {
  children: React.ReactNode;
  className?: string;
  level: 1 | 2 | 3 | 4;
  onClick?: () => void;
  style?: React.CSSProperties;
};

const Title = (props: Props) => <AntdTitle {...props} />;

export default Title;
