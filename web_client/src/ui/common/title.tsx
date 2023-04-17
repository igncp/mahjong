import { Typography } from "antd";

const { Title: AntdTitle } = Typography;

type Props = {
  children: React.ReactNode;
  level: 1 | 2 | 3 | 4;
  style?: React.CSSProperties;
};

const Title = (props: Props) => <AntdTitle {...props} />;

export default Title;
