import { Text, Divider, Grid, Group, Paper, ScrollArea, useMantineTheme } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { StatsWithSegments } from "@components/Shared/StatsWithSegments";
import { ColorInfo } from "@components/Shared/ColorInfo";
import { TransactionListItem } from "@components/DataDisplay/TransactionListItem";

export type AnalyticsTabProps = {
  value: TauriTypes.StockItemDetails | undefined;
};

export function AnalyticsTab({ value }: AnalyticsTabProps) {
  if (!value) return <></>;
  const theme = useMantineTheme();
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`stock_item_details.tabs.analytics.${key}`, { ...context }, i18Key);

  return (
    <Grid>
      <Grid.Col span={6}>
        <Paper>
          <Group p={10} justify="space-between">
            <Text>{useTranslateTab("last_transactions.title")}</Text>
            <Group>
              <ColorInfo
                infoProps={{
                  "data-color-mode": "bg",
                  "data-transaction-type": "purchase",
                }}
                text={useTranslateTab("last_transactions.info_box.purchase", { count: value.last_transactions.length || 0 })}
              />
              <ColorInfo
                infoProps={{
                  "data-color-mode": "bg",
                  "data-transaction-type": "sale",
                }}
                text={useTranslateTab("last_transactions.info_box.sale", { count: value.last_transactions.length || 0 })}
              />
            </Group>
          </Group>
          <Divider />
          <ScrollArea p={10}>
            {value.last_transactions.map((transaction, index) => (
              <TransactionListItem key={index} transaction={transaction} />
            ))}
          </ScrollArea>
        </Paper>
      </Grid.Col>
      <Grid.Col span={6}>
        <StatsWithSegments
          segments={[
            { label: "Expenses", tooltip: "Average Expense", count: value.expenses, part: value.average_expense, color: theme.colors.red[7] },
            { label: "Revenue", tooltip: "Average Revenue", count: value.revenue, part: value.average_revenue, color: theme.colors.green[7] },
            {
              label: "Total profit",
              tooltip: "Average Profit",
              count: value.total_profit,
              part: value.average_profit,
              color: theme.colors.violet[7],
            },
          ]}
          showPercent
          hidePercentBar
        />
        <StatsWithSegments
          segments={[
            {
              label: "Purchases",
              tooltip: "Average Expense",
              count: value.purchases_count,
              color: theme.colors.red[7],
            },
            { label: "Sales", tooltip: "Average Revenue", count: value.sale_count, color: theme.colors.green[7] },
            {
              label: "Total",
              tooltip: "Average Profit",
              count: value.total_transactions,
              part: value.profit_margin,
              color: theme.colors.violet[7],
            },
          ]}
          hidePercentBar
        />
      </Grid.Col>
    </Grid>
  );
}
