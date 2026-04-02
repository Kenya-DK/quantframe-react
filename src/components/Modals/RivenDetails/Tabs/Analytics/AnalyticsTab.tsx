import { Grid, ScrollArea, Title } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { TransactionListItem } from "@components/DataDisplay/TransactionListItem";
import { FinancialReportCard } from "@components/Shared/FinancialReportCard";

interface Properties {
  last_transactions: TauriTypes.TransactionDto[];
  report: TauriTypes.FinancialReport;
  [key: string]: any;
}

export type AnalyticsTabProps = {
  value: TauriTypes.StockItem<Properties> | TauriTypes.WishListItem<Properties> | undefined;
};

export function AnalyticsTab({ value }: AnalyticsTabProps) {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`item_details.tabs.analytics.${key}`, { ...context }, i18Key);
  if (!value) return <></>;

  return (
    <Grid mt="lg">
      <Grid.Col span={3.5}>
        <Title order={3} mb="md">
          {useTranslate("titles.last_transactions")}
        </Title>
        <ScrollArea>
          {value.properties.last_transactions.map((transaction, index) => (
            <TransactionListItem key={index} transaction={transaction} orientation="vertical" />
          ))}
        </ScrollArea>
      </Grid.Col>
      <Grid.Col span={8.5}>
        <Title order={3} mb="md">
          {useTranslate("titles.financial_summary")}
        </Title>
        <FinancialReportCard hideTradeCount data={value.properties.report} />
      </Grid.Col>
    </Grid>
  );
}
