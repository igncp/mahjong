import { Table as AntdTable } from "antd";
import { ColumnsType } from "antd/es/table";

type Props<A extends object> = {
  className?: string;
  dataSource: A[];
  columns: ColumnsType<A>;
  onRow?: (record: A) => React.HTMLAttributes<HTMLElement>;
};

const Table = <A extends object>(props: Props<A>) => <AntdTable {...props} />;

export default Table;
