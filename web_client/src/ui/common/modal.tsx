import { Modal as AntdModal } from "antd";

type Props = {
  children: React.ReactNode;
  footer?: null | React.ReactNode[];
  onCancel: () => void;
  open: boolean;
  title?: string;
};

const Modal = (props: Props) => <AntdModal {...props} />;

export default Modal;
