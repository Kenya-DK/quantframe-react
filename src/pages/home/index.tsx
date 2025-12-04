import {
  Box,
  Container,
  Text,
  Image,
  Divider,
  Grid,
  Group,
  Paper,
  ScrollArea,
  useMantineTheme,
  NumberFormatter,
  Tooltip,
  Stack,
} from "@mantine/core";
import api from "@api/index";
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
import { useTranslateBase, useTranslatePages } from "@hooks/useTranslate.hook";
import { TextTranslate } from "@components/Shared/TextTranslate";
import i18next from "i18next";
import { TauriTypes } from "$types";
import { DataTable } from "mantine-datatable";
import classes from "./Home.module.css";
import { StatsWithIcon } from "@components/Shared/StatsWithIcon";
import { BarCardChart } from "@components/Shared/BarCardChart";
import faMoneyBillTrendDown from "@icons/faMoneyBillTrendDown";
import { ColorInfo } from "@components/Shared/ColorInfo";
import { TransactionListItem } from "@components/DataDisplay/TransactionListItem";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { useAppContext } from "../../contexts/app.context";

const BarChartFooter = ({ i18nKey, statistics }: { i18nKey: string; statistics: TauriTypes.FinancialReport }) => {
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
        values={{ expense: statistics?.expenses || 0, revenue: statistics?.revenue || 0, profit: statistics?.total_profit || 0 }}
        components={ExtraComponents}
      />
      <TextTranslate
        i18nKey={`${i18nKey}.trades`}
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

export default function HomePage() {
  const theme = useMantineTheme();
  const { settings } = useAppContext();

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`home.${key}`, { ...context }, i18Key);
  const useTranslateCards = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`cards.${key}`, { ...context }, i18Key);

  const { data: summary, refetch: refetchSummary } = api.dashboard.summary();
  const handleRefresh = (_data: any) => {
    refetchSummary();
  };
  // Use the custom hook for Tauri events
  useTauriEvent(TauriTypes.Events.RefreshTransactions, handleRefresh, []);
  return (
    <Container size={"100%"}>
      <Grid className={classes.wrapper} data-has-alert={useHasAlert()}>
        <Grid.Col span={4}>
          {useTranslateBase("mods.impact_damage.short")}
          {settings && (
            <>
              <button onClick={async () => api.app.updateSettings({ ...settings, lang: "en" })}>English</button>
              <button onClick={async () => api.app.updateSettings({ ...settings, lang: "es" })}>Español</button>
            </>
          )}
          <StatsWithIcon
            count={summary?.total.total_profit || 0}
            color={theme.other.chartStyles.total.bgColor}
            title={useTranslateCards("total.title")}
            icon={<FontAwesomeIcon size="2x" icon={faMoneyBill} />}
            footer={
              <TextTranslate
                i18nKey={useTranslateCards("total.footer")}
                values={{
                  sales: summary?.total.sale_count || 0,
                  purchases: summary?.total.purchases_count || 0,
                  quantity: summary?.total.total_transactions || 0,
                  profit_margin: (summary?.total.profit_margin || 0).toFixed(2),
                }}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithIcon
            count={summary?.today.summary.total_profit || 0}
            color={theme.other.chartStyles.today.bgColor}
            title={useTranslateCards("today.title")}
            icon={<FontAwesomeIcon size="2x" icon={faCalendarAlt} />}
            footer={
              <TextTranslate
                i18nKey={useTranslateCards("today.footer")}
                values={{
                  sales: summary?.today.summary.sale_count || 0,
                  purchases: summary?.today.summary.purchases_count || 0,
                  quantity: summary?.today.summary.total_transactions || 0,
                  profit_margin: (summary?.today.summary.profit_margin || 0).toFixed(2),
                }}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithIcon
            count={summary?.best_seller.total_profit || 0}
            color={theme.other.chartStyles.lastDays.bgColor}
            title={useTranslateCards("best_seller.title")}
            icon={<FontAwesomeIcon size="2x" icon={faBoxOpen} />}
            footer={
              <TextTranslate
                i18nKey={useTranslateCards("best_seller.footer")}
                values={{
                  name: summary?.best_seller.properties.item_name || "",
                  sales: summary?.best_seller.sale_count || 0,
                  purchases: summary?.best_seller.purchases_count || 0,
                  quantity: summary?.best_seller.total_transactions || 0,
                  profit_margin: (summary?.best_seller.profit_margin || 0).toFixed(2),
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
            labels={i18next.t("months", { returnObjects: true }) as string[]}
            chartStyle={{ background: theme.other.chartStyles.total.bgColor, height: "200px" }}
            datasets={[
              {
                label: useTranslateCards("total.bar_chart.datasets.this_year"),
                data: summary?.total.present_year.chart.values || [],
                backgroundColor: theme.other.chartStyles.total.currentYearLineColor,
              },
              {
                label: useTranslateCards("total.bar_chart.datasets.last_year"),
                data: summary?.total.last_year.chart.values || [],
                backgroundColor: theme.other.chartStyles.total.lastYearLineColor,
              },
            ]}
            context={
              <BarChartFooter
                i18nKey={useTranslateCards("total.bar_chart.footers", undefined, true)}
                statistics={summary?.total as TauriTypes.FinancialReport}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <BarCardChart
            title={useTranslateCards("today.bar_chart.title")}
            labels={summary?.today.chart.labels || []}
            chartStyle={{ background: theme.other.chartStyles.today.bgColor, height: "200px" }}
            datasets={[
              {
                label: useTranslateCards("today.bar_chart.datasets.profit"),
                data: summary?.today.chart.values || [],
                backgroundColor: theme.other.chartStyles.today.lineColor,
              },
            ]}
            context={
              <BarChartFooter
                i18nKey={useTranslateCards("today.bar_chart.footers", undefined, true)}
                statistics={summary?.today.summary as TauriTypes.FinancialReport}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <BarCardChart
            title={useTranslateCards("recent_days.bar_chart.title", { days: (summary?.recent_days.chart.labels.length || 1) - 1 || 0 })}
            labels={summary?.recent_days.chart.labels || []}
            chartStyle={{ background: theme.other.chartStyles.lastDays.bgColor, height: "200px" }}
            datasets={[
              {
                label: useTranslateCards("recent_days.bar_chart.datasets.profit"),
                data: summary?.recent_days.chart.values || [],
                backgroundColor: theme.other.chartStyles.lastDays.lineColor,
              },
            ]}
            context={
              <BarChartFooter
                i18nKey={useTranslateCards("recent_days.bar_chart.footers", undefined, true)}
                statistics={summary?.recent_days.summary as TauriTypes.FinancialReport}
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
                    "data-transaction-type": "purchase",
                  }}
                  text={useTranslateCards("last_transaction.info_box.purchase", { count: summary?.total.purchases_count || 0 })}
                />
                <ColorInfo
                  infoProps={{
                    "data-color-mode": "bg",
                    "data-transaction-type": "sale",
                  }}
                  text={useTranslateCards("last_transaction.info_box.sale", { count: summary?.total.sale_count || 0 })}
                />
              </Group>
            </Group>
            <Divider />
            <ScrollArea className={classes.transactions} p={10} data-has-alert={useHasAlert()}>
              {summary?.resent_transactions.map((transaction, index) => (
                <TransactionListItem key={index} transaction={transaction} />
              ))}
            </ScrollArea>
          </Paper>
        </Grid.Col>
        <Grid.Col span={6}>
          <DataTable
            records={summary?.categories || []}
            idAccessor={"properties.name"}
            // define columns
            columns={[
              {
                accessor: "name",
                title: useTranslateCards("best_seller.by_category.datatable.columns.name"),
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
                title: useTranslateCards("best_seller.by_category.datatable.columns.revenue"),
                render: ({ revenue }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={revenue} />,
              },
              {
                accessor: "expense",
                title: useTranslateCards("best_seller.by_category.datatable.columns.expense"),
                render: ({ expenses }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={expenses} />,
              },
              {
                accessor: "profit",
                title: useTranslateCards("best_seller.by_category.datatable.columns.profit"),
                render: ({ total_profit }) => (
                  <NumberFormatter
                    style={{ color: total_profit > 0 ? theme.other.positiveColor : theme.other.negativeColor } as React.CSSProperties}
                    thousandSeparator="."
                    decimalSeparator=","
                    value={total_profit}
                  />
                ),
              },
              {
                accessor: "profit_margin",
                title: useTranslateCards("best_seller.by_category.datatable.columns.profit_margin"),
                render: ({ profit_margin }) => <NumberFormatter decimalScale={2} suffix=" %" value={profit_margin} />,
              },
            ]}
          />
        </Grid.Col>
      </Grid>
    </Container>
  );
}
