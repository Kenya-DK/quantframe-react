import { TauriTypes } from "$types";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { StatsWithSegments } from "../StatsWithSegments";
import { Stack } from "@mantine/core";

export interface FinancialReportCardProps {
  data: TauriTypes.FinancialReport | undefined;
  hideComponents?: ("total_transactions" | "trade_count" | "revenue" | "expenses" | "total_profit" | "highest_revenue" | "highest_expense")[];
  hideTradeCount?: boolean;
  loading?: boolean;
  hidePercentBar?: boolean;
}

export const FinancialReportCard = ({ data, hideComponents, hideTradeCount, hidePercentBar }: FinancialReportCardProps) => {
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`financial_report_card.${key}`, { ...context }, i18Key);

  return (
    <StatsWithSegments
      p={0}
      orientation="vertical"
      h={320}
      hidePercentBar={hidePercentBar}
      segments={[
        {
          label: useTranslate("labels.total_transactions"),
          count: data?.total_transactions || 0,
          color: "orange",
          tooltip: useTranslate("tooltips.total_credits"),
          part: data?.properties?.total_credits || 0,
          hideInProgress: true,
          hide: hideComponents?.includes("total_transactions"),
          suffix: " C",
          decimalScale: 2,
        },
        {
          label: useTranslate("labels.trade_count"),
          count: data?.properties?.total_trades || 0,
          color: "var(--qf-transaction-type-trade)",
          hide: hideComponents?.includes("trade_count") || hideTradeCount,
        },
        {
          label: useTranslate("labels.revenue"),
          count: data?.sale_count || 0,
          color: "green",
          part: data?.revenue || 0,
          usePartForPercentage: true,
          tooltip: useTranslate("tooltips.total_revenue"),
          hide: hideComponents?.includes("revenue"),
          suffix: " P",
        },
        {
          label: useTranslate("labels.expenses"),
          count: data?.purchases_count || 0,
          color: "red",
          part: data?.expenses || 0,
          usePartForPercentage: true,
          tooltip: useTranslate("tooltips.total_expenses"),
          hide: hideComponents?.includes("expenses"),
          suffix: " P",
        },
        {
          label: useTranslate("labels.total_profit"),
          count: data?.total_profit || 0,
          color: "teal",
          hideInProgress: true,
          tooltip: useTranslate("tooltips.average_profit"),
          part: data?.average_profit || 0,
          hide: hideComponents?.includes("total_profit"),
          suffix: " P",
          decimalScale: 2,
        },
      ]}
      showPercent
      footer={
        <Stack>
          <StatsWithSegments
            p={0}
            hidePercentBar
            segments={[
              {
                label: useTranslate("labels.highest_revenue"),
                count: data?.highest_revenue || 0,
                color: "green",
                tooltip: useTranslate("tooltips.lowest_revenue"),
                part: data?.lowest_revenue || 0,
                decimalScale: 2,
                hide: hideComponents?.includes("highest_revenue"),
              },
              {
                label: useTranslate("labels.highest_expense"),
                count: data?.highest_expense || 0,
                color: "red",
                tooltip: useTranslate("tooltips.lowest_expense"),
                part: data?.lowest_expense || 0,
                decimalScale: 2,
                hide: hideComponents?.includes("highest_expense"),
              },
            ]}
            showPercent
          />
        </Stack>
      }
    />
  );
};
