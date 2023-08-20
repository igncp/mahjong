import { Modal as AntdModal } from "antd";

type Props = {
  title?: string;
  onCancel: () => void;
  open: boolean;
  children: React.ReactNode;
  footer?: React.ReactNode[] | null;
};

const Modal = (props: Props) => <AntdModal {...props} />;

export default Modal;
