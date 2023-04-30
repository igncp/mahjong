import { Alert as AntdAlert } from "antd";

type Props = {
  message: React.ReactNode;
  type?: "success" | "info" | "warning" | "error";
};

const Alert = (props: Props) => <AntdAlert {...props} />;

export default Alert;
