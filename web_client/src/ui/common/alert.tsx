import { Alert as AntdAlert } from "antd";

type Props = {
  message: React.ReactNode;
  style?: React.CSSProperties;
  type?: "error" | "info" | "success" | "warning";
};

const Alert = (props: Props) => <AntdAlert {...props} />;

export default Alert;
