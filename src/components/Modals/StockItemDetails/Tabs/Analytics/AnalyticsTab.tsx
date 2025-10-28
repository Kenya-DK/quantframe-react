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
            {
              label: "Expenses",
              tooltip: "Average Expense",
              count: value.report.expenses,
              part: value.report.average_expense,
              color: theme.colors.red[7],
            },
            {
              label: "Revenue",
              tooltip: "Average Revenue",
              count: value.report.revenue,
              part: value.report.average_revenue,
              color: theme.colors.green[7],
            },
            {
              label: "Total profit",
              tooltip: "Average Profit",
              count: value.report.total_profit,
              part: value.report.average_profit,
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
              count: value.report.purchases_count,
              color: theme.colors.red[7],
            },
            { label: "Sales", tooltip: "Average Revenue", count: value.report.sale_count, color: theme.colors.green[7] },
            {
              label: "Total",
              tooltip: "Average Profit",
              count: value.report.total_transactions,
              part: value.report.profit_margin,
              color: theme.colors.violet[7],
            },
          ]}
          hidePercentBar
        />
      </Grid.Col>
    </Grid>
  );
}
