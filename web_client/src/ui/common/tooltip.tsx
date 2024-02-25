import dynamic from "next/dynamic";

const AntdTooltip = dynamic(() => import("antd").then((m) => m.Tooltip), {
  ssr: false,
});

type Props = {
  children: React.ReactNode;
  title: React.ReactNode;
};

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore React typings
const Tooltip = (props: Props) => <AntdTooltip {...props} />;

export default Tooltip;
