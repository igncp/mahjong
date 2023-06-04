import { Tooltip as AntdTooltip } from "antd";

type Props = {
  children: React.ReactNode;
  title: string;
};

const Tooltip = (props: Props) => <AntdTooltip {...props} />;

export default Tooltip;
