import {
  Container,
  Grid,
  Group,
  NumberFormatter,
  Paper,
  Stack,
  Tooltip,
  getGradient,
  useMantineTheme,
  Divider,
  ScrollArea,
  Text,
} from "@mantine/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faBoxOpen,
  faCalendarAlt,
  faCartShopping,
  faHandHoldingDollar,
  faHandshake,
  faMoneyBill,
  faMoneyBillTrendUp,
  faSackDollar,
} from "@fortawesome/free-solid-svg-icons";
import { useWarframeMarketContextContext } from "@contexts/warframeMarket.context";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { getCssVariable } from "@utils/helper";
import { DataTable } from "mantine-datatable";
import { useEffect, useState } from "react";
import { TauriTypes } from "$types";
import { TextTranslate } from "@components/TextTranslate";
import { StatsWithIcon } from "@components/StatsWithIcon";
import { BarCardChart } from "@components/BarCardChart";
import { ColorInfo } from "@components/ColorInfo";
import classes from "./Home.module.css";
import { TransactionListItem } from "@components/TransactionListItem";
import faMoneyBillTrendDown from "@icons/faMoneyBillTrendDown";

const BarChartFooter = ({ i18nKey, statistics }: { i18nKey: string; statistics: TauriTypes.StatisticProfitBase }) => {
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`home.${key}`, { ...context }, i18Key);
  const useTranslateTooltips = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tooltips.bar_chart.footer.${key}`, { ...context }, i18Key);

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
        i18nKey={`${i18nKey}.profit`}
        values={{ expense: statistics?.expense || 0, revenue: statistics?.revenue || 0, profit: statistics?.profit || 0 }}
        components={ExtraComponents}
      />
      <TextTranslate
        i18nKey={`${i18nKey}.trades`}
        values={{
          purchases: statistics?.purchases || 0,
          sales: statistics?.sales || 0,
          trades: (statistics?.sales || 0) + (statistics?.purchases || 0),
        }}
        components={ExtraComponents}
      />
    </Stack>
  );
};
export default function HomePage() {
  const theme = useMantineTheme();
  // State's
  const [purchaseCount, setPurchaseCount] = useState(0);
  const [saleCount, setSaleCount] = useState(0);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`home.${key}`, { ...context }, i18Key);
  const useTranslateCards = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`cards.${key}`, { ...context }, i18Key);
  const { statistics } = useWarframeMarketContextContext();

  useEffect(() => {
    if (!statistics) return;
    setPurchaseCount(
      statistics?.recent_transactions.transactions.filter((transaction) => transaction.transaction_type === TauriTypes.TransactionType.Purchase)
        .length
    );
    setSaleCount(
      statistics?.recent_transactions.transactions.filter((transaction) => transaction.transaction_type === TauriTypes.TransactionType.Sale).length
    );
  }, [statistics]);
  return (
    <Container size={"100%"}>
      <Grid>
        <Grid.Col span={4}>
          <StatsWithIcon
            count={statistics?.total.profit || 0}
            color={getGradient({ deg: 180, from: "green.7", to: "green.9" }, theme)}
            title={useTranslateCards("total.title")}
            icon={<FontAwesomeIcon size="2x" icon={faMoneyBill} />}
            footer={
              <TextTranslate
                i18nKey={useTranslateCards("total.footer")}
                values={{
                  sales: statistics?.total.sales || 0,
                  purchases: statistics?.total.purchases || 0,
                  quantity: (statistics?.total.sales || 0) + (statistics?.total?.purchases || 0),
                  profit_margin: ((statistics?.total.profit_margin || 0) * 100).toFixed(2),
                }}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithIcon
            count={statistics?.today.profit || 0}
            color={getGradient({ deg: 180, from: "grape.7", to: "grape.9" }, theme)}
            title={useTranslateCards("today.title")}
            icon={<FontAwesomeIcon size="2x" icon={faCalendarAlt} />}
            footer={
              <TextTranslate
                i18nKey={useTranslateCards("today.footer")}
                values={{
                  sales: statistics?.today.sales || 0,
                  purchases: statistics?.today.purchases || 0,
                  quantity: (statistics?.today.sales || 0) + (statistics?.today.purchases || 0),
                  profit_margin: ((statistics?.today.profit_margin || 0) * 100).toFixed(2),
                }}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithIcon
            count={statistics?.best_seller?.items[0]?.profit || 0}
            color={getGradient({ deg: 180, from: "blue.7", to: "blue.9" }, theme)}
            title={useTranslateCards("best_seller.title")}
            icon={<FontAwesomeIcon size="2x" icon={faBoxOpen} />}
            footer={
              <TextTranslate
                i18nKey={useTranslateCards("best_seller.footer")}
                values={{
                  name: statistics?.best_seller.items[0]?.name || "",
                  sales: statistics?.best_seller.items[0]?.sales || 0,
                  purchases: statistics?.best_seller.items[0]?.purchases || 0,
                  quantity: statistics?.best_seller.items[0]?.quantity || 0,
                  profit_margin: ((statistics?.best_seller.items[0]?.profit_margin || 0) * 100).toFixed(2),
                }}
              />
            }
          />
        </Grid.Col>
      </Grid>
      <Grid>
        <Grid.Col span={4}>
          <BarCardChart
            title={useTranslateCards("total.bar_chart.title")}
            labels={statistics?.total.labels || []}
            chartStyle={{ background: getGradient({ deg: 180, from: "green.8", to: "green.9" }, theme), height: "200px" }}
            datasets={[
              {
                label: useTranslateCards("total.bar_chart.datasets.this_year"),
                data: statistics?.total.present.profit_values || [],
                backgroundColor: getCssVariable("--mantine-color-blue-3"),
              },
              {
                label: useTranslateCards("total.bar_chart.datasets.last_year"),
                data: statistics?.total.previous.profit_values || [],
                backgroundColor: getCssVariable("--mantine-color-blue-7"),
              },
            ]}
            context={
              <BarChartFooter
                i18nKey={useTranslateCards("total.bar_chart.footers", undefined, true)}
                statistics={statistics?.total as TauriTypes.StatisticProfitBase}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <BarCardChart
            title={useTranslateCards("today.bar_chart.title")}
            labels={statistics?.today.chart_profit.labels || []}
            chartStyle={{ background: getGradient({ deg: 180, from: "grape.8", to: "grape.9" }, theme), height: "200px" }}
            datasets={[
              {
                label: useTranslateCards("today.bar_chart.datasets.profit"),
                data: statistics?.today.chart_profit.profit_values || [],
                backgroundColor: getCssVariable("--profit-bar-color"),
              },
            ]}
            context={
              <BarChartFooter
                i18nKey={useTranslateCards("today.bar_chart.footers", undefined, true)}
                statistics={statistics?.today as TauriTypes.StatisticProfitBase}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <BarCardChart
            title={useTranslateCards("recent_days.bar_chart.title", { days: statistics?.recent_days.days })}
            labels={statistics?.recent_days.chart_profit.labels || []}
            chartStyle={{ background: getGradient({ deg: 180, from: "blue.8", to: "blue.9" }, theme), height: "200px" }}
            datasets={[
              {
                label: useTranslateCards("recent_days.bar_chart.datasets.profit"),
                data: statistics?.recent_days.chart_profit.profit_values || [],
                backgroundColor: getCssVariable("--profit-bar-color"),
              },
            ]}
            context={
              <BarChartFooter
                i18nKey={useTranslateCards("recent_days.bar_chart.footers", undefined, true)}
                statistics={statistics?.recent_days as TauriTypes.StatisticProfitBase}
              />
            }
          />
        </Grid.Col>
      </Grid>
      <Grid>
        <Grid.Col span={6}>
          <Paper>
            <Group p={10} justify="space-between">
              <Text>{useTranslateCards("last_transaction.title")}</Text>
              <Group>
                <ColorInfo
                  infoProps={{
                    "data-color-mode": "bg",
                    "data-trade-type": "purchase",
                  }}
                  text={useTranslateCards("last_transaction.info_box.purchase", { count: purchaseCount })}
                />
                <ColorInfo
                  infoProps={{
                    "data-color-mode": "bg",
                    "data-trade-type": "sale",
                  }}
                  text={useTranslateCards("last_transaction.info_box.sale", { count: saleCount })}
                />
              </Group>
            </Group>
            <Divider />
            <ScrollArea className={classes.transactions} p={10} data-has-alert={useHasAlert()}>
              {statistics?.recent_transactions.transactions.map((transaction, index) => (
                <TransactionListItem key={index} transaction={transaction} />
              ))}
            </ScrollArea>
          </Paper>
        </Grid.Col>
        <Grid.Col span={6}>
          <DataTable
            records={statistics?.best_seller.category || []}
            idAccessor={"name"}
            // define columns
            columns={[
              {
                accessor: "name",
                title: useTranslateCards("best_seller.by_category.datatable.columns.name"),
              },
              {
                accessor: "revenue",
                title: useTranslateCards("best_seller.by_category.datatable.columns.revenue"),
                render: ({ revenue }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={revenue} />,
              },
              {
                accessor: "expense",
                title: useTranslateCards("best_seller.by_category.datatable.columns.expense"),
                render: ({ expense }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={expense} />,
              },
              {
                accessor: "profit",
                title: useTranslateCards("best_seller.by_category.datatable.columns.profit"),
                customCellAttributes: ({ profit }) => ({
                  "data-color-mode": "text",
                  "data-profit": profit > 0 ? "positive" : "negative",
                }),
                render: ({ profit }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={profit} />,
                // render: ({ profit }) => <Text color={profit > 0 ? getTradeClassificationColorCode(Wfm.TradeClassification.Sell) : getTradeClassificationColorCode(Wfm.TradeClassification.Buy)}>{profit}</Text>,
              },
              {
                accessor: "profit_margin",
                title: useTranslateCards("best_seller.by_category.datatable.columns.profit_margin"),
                render: ({ profit_margin }) => <NumberFormatter decimalScale={2} suffix=" %" value={profit_margin * 100} />,
              },
            ]}
          />
        </Grid.Col>
      </Grid>
    </Container>
  );
}
