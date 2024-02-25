import { Select as AntdSelect } from "antd";

export type SelectOption = {
  label: string;
  value: string;
};

type Props = {
  defaultValue: string;
  disabled: boolean;
  onChange: (value: string) => void;
  options: SelectOption[];
  style: React.CSSProperties;
};

const Select = (props: Props) => <AntdSelect {...props} />;

export default Select;
