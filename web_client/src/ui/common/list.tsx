import { List as AntdList } from "antd";

type Props<A> = {
  bordered?: boolean;
  dataSource: A[];
  header?: React.ReactNode;
  renderItem: (item: A) => JSX.Element;
};

const List = <A,>(props: Props<A>) => <AntdList {...props} />;

type ItemProps = {
  children: React.ReactNode;
};

export const ListItem = (props: ItemProps) => <AntdList.Item {...props} />;

export default List;
