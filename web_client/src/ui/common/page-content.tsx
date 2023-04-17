import Space from "./space";

type Props = {
  children: React.ReactNode;
  style?: React.CSSProperties;
};

const PageContent = (props: Props) => (
  <Space
    {...props}
    direction="vertical"
    style={{ padding: "0 10px", width: "calc(100% - 20px)", ...props.style }}
  />
);

export default PageContent;
