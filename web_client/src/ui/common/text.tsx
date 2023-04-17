import { Typography } from "antd";

const { Text: AntdText } = Typography;

type Props = {
  children: React.ReactNode;
  style?: React.CSSProperties;
};

const Text = (props: Props) => <AntdText {...props} />;

export default Text;
