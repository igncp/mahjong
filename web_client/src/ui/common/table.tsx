import { Table as AntdTable } from "antd";
import type { ColumnsType } from "antd/es/table";

type Props<A extends object> = {
  className?: string;
  columns: ColumnsType<A>;
  dataSource: A[];
  onRow?: (record: A) => React.HTMLAttributes<HTMLElement>;
};

const Table = <A extends object>(props: Props<A>) => <AntdTable {...props} />;

export default Table;
