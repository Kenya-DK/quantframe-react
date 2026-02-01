import { TauriTypes } from "$types";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { StatsWithSegments } from "../StatsWithSegments";

export interface FinancialReportCardProps {
  data: TauriTypes.FinancialReport | undefined;
  hideTradeCount?: boolean;
  loading?: boolean;
}

export const FinancialReportCard = ({ data, hideTradeCount }: FinancialReportCardProps) => {
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`financial_report_card.${key}`, { ...context }, i18Key);

  return (
    <StatsWithSegments
      p={0}
      orientation="vertical"
      hidePercentBar
      segments={[
        {
          label: useTranslate("labels.total_transactions"),
          count: data?.total_transactions || 0,
          color: "orange",
          tooltip: useTranslate("tooltips.total_credits"),
          part: data?.properties?.total_credits || 0,
          suffix: " C",
          decimalScale: 2,
        },
        {
          label: useTranslate("labels.trade_count"),
          count: data?.properties?.total_trades || 0,
          color: "var(--qf-transaction-type-trade)",
          hide: hideTradeCount,
          part: null,
        },
        {
          label: useTranslate("labels.purchases_count"),
          count: data?.purchases_count || 0,
          color: "var(--qf-transaction-type-purchase)",
          part: data?.expenses || 0,
          suffix: " P",
        },
        {
          label: useTranslate("labels.sales_count"),
          count: data?.sale_count || 0,
          color: "var(--qf-transaction-type-sale)",
          part: data?.revenue || 0,
          suffix: " P",
        },
        {
          label: useTranslate("labels.total_profit"),
          count: data?.total_profit || 0,
          color: "teal",
          tooltip: useTranslate("tooltips.profit_margin"),
          part: data?.profit_margin || 0,
          decimalScale: 2,
        },
      ]}
      showPercent
      percentSymbol="%"
    />
  );
};
