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
  Image,
  Box,
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
import i18next from "i18next";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { getCssVariable } from "@utils/helper";
import { DataTable } from "mantine-datatable";
import { TauriTypes } from "$types";
import { TextTranslate } from "@components/TextTranslate";
import { StatsWithIcon } from "@components/StatsWithIcon";
import { BarCardChart } from "@components/BarCardChart";
import { ColorInfo } from "@components/ColorInfo";
import classes from "./Home.module.css";
import { TransactionListItem } from "@components/TransactionListItem";
import faMoneyBillTrendDown from "@icons/faMoneyBillTrendDown";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";

const BarChartFooter = ({ i18nKey, statistics }: { i18nKey: string; statistics: TauriTypes.TransactionSummaryDto }) => {
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
        values={{ expense: statistics?.expenses || 0, revenue: statistics?.revenue || 0, profit: statistics?.profit || 0 }}
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

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`home.${key}`, { ...context }, i18Key);
  const useTranslateCards = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`cards.${key}`, { ...context }, i18Key);

  // Queys
  let { data } = useQuery({
    queryKey: ["matrix_overview"],
    queryFn: () => api.summary.overview(),
    refetchOnWindowFocus: true,
  });
  return (
    <Container size={"100%"}>
      <Grid>
        <Grid.Col span={4}>
          <StatsWithIcon
            count={data?.total.profit || 0}
            color={getGradient({ deg: 180, from: "green.7", to: "green.9" }, theme)}
            title={useTranslateCards("total.title")}
            icon={<FontAwesomeIcon size="2x" icon={faMoneyBill} />}
            footer={
              <TextTranslate
                i18nKey={useTranslateCards("total.footer")}
                values={{
                  sales: data?.total.sales || 0,
                  purchases: data?.total.purchases || 0,
                  quantity: data?.total.total_transactions || 0,
                  profit_margin: (data?.total.profit_margin || 0).toFixed(2),
                }}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithIcon
            count={data?.today.profit || 0}
            color={getGradient({ deg: 180, from: "grape.7", to: "grape.9" }, theme)}
            title={useTranslateCards("today.title")}
            icon={<FontAwesomeIcon size="2x" icon={faCalendarAlt} />}
            footer={
              <TextTranslate
                i18nKey={useTranslateCards("today.footer")}
                values={{
                  sales: data?.today.sales || 0,
                  purchases: data?.today.purchases || 0,
                  quantity: data?.today.total_transactions || 0,
                  profit_margin: (data?.today.profit_margin || 0).toFixed(2),
                }}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithIcon
            count={data?.best_selling_items[0]?.profit || 0}
            color={getGradient({ deg: 180, from: "blue.7", to: "blue.9" }, theme)}
            title={useTranslateCards("best_seller.title")}
            icon={<FontAwesomeIcon size="2x" icon={faBoxOpen} />}
            footer={
              <TextTranslate
                i18nKey={useTranslateCards("best_seller.footer")}
                values={{
                  name: data?.best_selling_items[0]?.item_name || "",
                  sales: data?.best_selling_items[0]?.sales || 0,
                  purchases: data?.best_selling_items[0]?.purchases || 0,
                  quantity: data?.best_selling_items[0]?.quantity || 0,
                  profit_margin: ((data?.best_selling_items[0]?.profit_margin || 0) * 100).toFixed(2),
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
            // labels={data?.total.present_year.chart.labels || []}
            chartStyle={{ background: getGradient({ deg: 180, from: "green.8", to: "green.9" }, theme), height: "200px" }}
            datasets={[
              {
                label: useTranslateCards("total.bar_chart.datasets.this_year"),
                data: data?.total.present_year.chart.values || [],
                backgroundColor: getCssVariable("--mantine-color-blue-3"),
              },
              {
                label: useTranslateCards("total.bar_chart.datasets.last_year"),
                data: data?.total.last_year.chart.values || [],
                backgroundColor: getCssVariable("--mantine-color-blue-7"),
              },
            ]}
            context={
              <BarChartFooter
                i18nKey={useTranslateCards("total.bar_chart.footers", undefined, true)}
                statistics={data?.total as TauriTypes.TransactionSummaryDto}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <BarCardChart
            title={useTranslateCards("today.bar_chart.title")}
            labels={data?.today.chart.labels || []}
            chartStyle={{ background: getGradient({ deg: 180, from: "grape.8", to: "grape.9" }, theme), height: "200px" }}
            datasets={[
              {
                label: useTranslateCards("today.bar_chart.datasets.profit"),
                data: data?.today.chart.values || [],
                backgroundColor: getCssVariable("--profit-bar-color"),
              },
            ]}
            context={
              <BarChartFooter
                i18nKey={useTranslateCards("today.bar_chart.footers", undefined, true)}
                statistics={data?.today as TauriTypes.TransactionSummaryDto}
              />
            }
          />
        </Grid.Col>
        <Grid.Col span={4}>
          <BarCardChart
            title={useTranslateCards("recent_days.bar_chart.title", { days: data?.recent_days.chart.labels.length || 0 })}
            labels={data?.recent_days.chart.labels || []}
            chartStyle={{ background: getGradient({ deg: 180, from: "blue.8", to: "blue.9" }, theme), height: "200px" }}
            datasets={[
              {
                label: useTranslateCards("recent_days.bar_chart.datasets.profit"),
                data: data?.recent_days.chart.values || [],
                backgroundColor: getCssVariable("--profit-bar-color"),
              },
            ]}
            context={
              <BarChartFooter
                i18nKey={useTranslateCards("recent_days.bar_chart.footers", undefined, true)}
                statistics={data?.recent_days as TauriTypes.TransactionSummaryDto}
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
                  text={useTranslateCards("last_transaction.info_box.purchase", { count: data?.total.purchases || 0 })}
                />
                <ColorInfo
                  infoProps={{
                    "data-color-mode": "bg",
                    "data-trade-type": "sale",
                  }}
                  text={useTranslateCards("last_transaction.info_box.sale", { count: data?.total.sales || 0 })}
                />
              </Group>
            </Group>
            <Divider />
            <ScrollArea className={classes.transactions} p={10} data-has-alert={useHasAlert()}>
              {data?.resent_transactions.map((transaction, index) => (
                <TransactionListItem key={index} transaction={transaction} />
              ))}
            </ScrollArea>
          </Paper>
        </Grid.Col>
        <Grid.Col span={6}>
          <DataTable
            records={data?.category_summary || []}
            idAccessor={"name"}
            // define columns
            columns={[
              {
                accessor: "name",
                title: useTranslateCards("best_seller.by_category.datatable.columns.name"),
                width: "150px",
                render: ({ name, icon }) => (
                  <Box style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                    <Image src={icon} fallbackSrc="/question.png" radius="md" h={32} w={28} fit="contain" />
                    <Text>{name}</Text>
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
                customCellAttributes: ({ profit }) => ({
                  "data-color-mode": "text",
                  "data-profit": profit > 0 ? "positive" : "negative",
                }),
                render: ({ profit }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={profit} />,
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
