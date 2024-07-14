import { List as AntdList } from "antd";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";

type Props<A> = {
  bordered?: boolean;
  className?: string;
  dataSource: A[];
  header?: React.ReactNode;
  renderItem: (item: A) => JSX.Element;
  style?: React.CSSProperties;
};

const List = <A,>(props: Props<A>) => {
  const { t } = useTranslation();

  const emptyText = t("list.empty");

  const locale = useMemo(
    () => ({
      emptyText,
    }),
    [emptyText]
  );

  return <AntdList locale={locale} {...props} />;
};

type ItemProps = {
  children: React.ReactNode;
  className?: string;
  style?: React.CSSProperties;
};

export const ListItem = (props: ItemProps) => <AntdList.Item {...props} />;

export default List;
