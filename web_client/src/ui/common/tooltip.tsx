import { Tooltip as AntdTooltip } from "antd";

type Props = {
  children: React.ReactNode;
  title: React.ReactNode;
};

const Tooltip = (props: Props) => <AntdTooltip {...props} />;

export default Tooltip;
