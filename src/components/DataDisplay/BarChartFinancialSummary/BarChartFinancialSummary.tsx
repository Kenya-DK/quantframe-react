import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { TauriTypes } from "$types";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { Stack, Tooltip } from "@mantine/core";
import { faCartShopping, faHandHoldingDollar, faHandshake, faMoneyBillTrendUp, faSackDollar } from "@fortawesome/free-solid-svg-icons";
import { faMoneyBillTrendDown } from "@icons";
import { TextTranslate } from "../../Shared/TextTranslate";

export const BarChartFinancialSummary = ({ statistics }: { statistics: TauriTypes.FinancialReport | undefined }) => {
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`bar_chart_financial_summary.${key}`, { ...context }, i18Key);
  const useTranslateTooltips = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tooltips.${key}`, { ...context }, i18Key);
  const ExtraComponents = {
    expenseIco: (
      <Tooltip label={useTranslateTooltips("expense")}>
        <FontAwesomeIcon icon={faMoneyBillTrendDown} />
      </Tooltip>
    ),
    revenueIco: (
      <Tooltip label={useTranslateTooltips("revenue")}>
        <FontAwesomeIcon icon={faMoneyBillTrendUp} />
      </Tooltip>
    ),
    profitIco: (
      <Tooltip label={useTranslateTooltips("profit")}>
        <FontAwesomeIcon icon={faSackDollar} />
      </Tooltip>
    ),
    tradeIco: (
      <Tooltip label={useTranslateTooltips("trades")}>
        <FontAwesomeIcon icon={faHandshake} />
      </Tooltip>
    ),
    purchaseIco: (
      <Tooltip label={useTranslateTooltips("purchases")}>
        <FontAwesomeIcon icon={faCartShopping} />
      </Tooltip>
    ),
    saleIco: (
      <Tooltip label={useTranslateTooltips("sales")}>
        <FontAwesomeIcon icon={faHandHoldingDollar} />
      </Tooltip>
    ),
  };

  return (
    <Stack gap={"xs"}>
      <TextTranslate
        style={{ display: "flex", gap: "4px", alignItems: "center" }}
        i18nKey={useTranslate("profit", undefined, true)}
        values={{ expense: statistics?.expenses || 0, revenue: statistics?.revenue || 0, profit: statistics?.total_profit || 0 }}
        components={ExtraComponents}
      />
      <TextTranslate
        i18nKey={useTranslate("trades", undefined, true)}
        values={{
          purchases: statistics?.purchases_count || 0,
          sales: statistics?.sale_count || 0,
          trades: (statistics?.sale_count || 0) + (statistics?.purchases_count || 0),
        }}
        components={ExtraComponents}
      />
    </Stack>
  );
};
