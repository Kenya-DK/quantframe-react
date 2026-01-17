import { Box, Image, Text, NumberFormatter, useMantineTheme } from "@mantine/core";
import { DataTable } from "mantine-datatable";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";

export type BestByCategoryTableProps = {
  records?: TauriTypes.FinancialCategoryReport[];
};

export const BestByCategoryTable = ({ records = [] }: BestByCategoryTableProps) => {
  const theme = useMantineTheme();
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`best_by_category_table.${key}`, { ...context }, i18Key);

  return (
    <DataTable
      records={records}
      idAccessor="properties.name"
      columns={[
        {
          accessor: "name",
          title: useTranslate("datatable.columns.name"),
          width: "150px",
          render: ({ properties }) => (
            <Box style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <Image src={properties.icon} fallbackSrc="/question.png" radius="md" h={32} w={28} fit="contain" />
              <Text>{properties.name}</Text>
            </Box>
          ),
        },
        {
          accessor: "revenue",
          title: useTranslate("datatable.columns.revenue"),
          render: ({ revenue }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={revenue} />,
        },
        {
          accessor: "expense",
          title: useTranslate("datatable.columns.expense"),
          render: ({ expenses }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={expenses} />,
        },
        {
          accessor: "profit",
          title: useTranslate("datatable.columns.profit"),
          render: ({ total_profit }) => (
            <NumberFormatter
              style={{
                color: total_profit > 0 ? theme.other.positiveColor : theme.other.negativeColor,
              }}
              thousandSeparator="."
              decimalSeparator=","
              value={total_profit}
            />
          ),
        },
        {
          accessor: "profit_margin",
          title: useTranslate("datatable.columns.profit_margin"),
          render: ({ profit_margin }) => <NumberFormatter decimalScale={2} suffix=" %" value={profit_margin} />,
        },
        {
          accessor: "quantity",
          title: useTranslate("datatable.columns.quantity"),
          render: ({ total_transactions }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={total_transactions} />,
        },
      ]}
    />
  );
};
