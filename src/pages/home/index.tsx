import { Grid, useMantineTheme, Text, Container, Image } from "@mantine/core";
import { StatsWithIcon } from "@components/stats/statsWithIcon.stats";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCalendarAlt, faMoneyBill } from "@fortawesome/free-solid-svg-icons";
import { useTranslatePage, useTranslateGeneral } from "@hooks/index";
import { TransactionProfitChart } from "@components/stats/transactionProfitChart.stats";
import { useCacheContext, useWarframeMarketContextContext } from "@contexts/index";
import { wfmThumbnail } from "@api/index";
import { TextColor } from "@components/textColor";
import { DataTable } from "mantine-datatable";

export default function HomePage() {
  const theme = useMantineTheme();
  const translateBase = (key: string, context?: { [key: string]: any }) => useTranslatePage(`home.${key}`, { ...context })
  const { statistics } = useWarframeMarketContextContext();
  const { images_map } = useCacheContext();
  return (
    <Container size={"100%"}>
      {statistics &&
        <>
          <Grid>
            <Grid.Col md={4} >
              <StatsWithIcon
                color="linear-gradient(195deg, rgb(102, 187, 106), rgb(67, 160, 71))"
                icon={<FontAwesomeIcon icon={faMoneyBill} size="2x" />}
                title={translateBase("stats_cards.total.title")}
                count={statistics.total.profit}
                fotter={
                  <TextColor
                    i18nKey={translateBase("stats_cards.total.context")}
                    values={{
                      sales: statistics.total.sales,
                      purchases: statistics.total.purchases,
                      quantity: statistics.total.sales + statistics.total.purchases,
                      profit_margin: ((statistics.today.profit_margin || 0) * 100).toPrecision(2),
                    }}
                  />}
              />
            </Grid.Col>
            <Grid.Col md={4} >
              <StatsWithIcon
                color="linear-gradient(195deg, rgb(236, 64, 122), rgb(216, 27, 96))"
                icon={<FontAwesomeIcon icon={faCalendarAlt} size="2x" />}
                title={translateBase("stats_cards.today.title")}
                count={statistics.today.profit}
                fotter={
                  <TextColor
                    i18nKey={translateBase("stats_cards.today.context")}
                    values={{
                      sales: statistics.today.sales,
                      purchases: statistics.today.purchases,
                      quantity: statistics.today.sales + statistics.today.purchases,
                      profit_margin: ((statistics.today.profit_margin || 0) * 100).toPrecision(2),
                    }}
                  />}
              />
            </Grid.Col>
            <Grid.Col md={4} >
              <StatsWithIcon
                color="linear-gradient(195deg, rgb(154 64 236), rgb(117 27 216))"
                icon={<Image width={60} src={wfmThumbnail(images_map[statistics.best_seller.items[0]?.url || ""])} />}
                title={translateBase("stats_cards.best_selling.title")}
                count={statistics.best_seller.items[0]?.profit || 0}
                fotter={
                  <TextColor
                    i18nKey={translateBase("stats_cards.best_selling.context")}
                    values={{
                      name: statistics.best_seller.items[0]?.name || "",
                      sales: statistics.best_seller.items[0]?.sales || 0,
                      purchases: statistics.best_seller.items[0]?.purchases || 0,
                      quantity: statistics.best_seller.items[0]?.quantity || 0,
                      profit_margin: ((statistics.best_seller.items[0]?.profit_margin || 0) * 100).toPrecision(2),
                    }}
                  />}
              />
            </Grid.Col>
          </Grid>
          <Grid mt={10}>
            <Grid.Col md={4} >
              <TransactionProfitChart
                title={translateBase("stats_cards.total_chart.title")}
                showDatasetLabels
                type="revenue"
                // Green to light green
                background="linear-gradient(195deg, #0a4e0a, #00a300)"
                labels={statistics.total.labels || []}
                orderWithRevenues={[
                  { ...statistics.total.present, labels: statistics.total.labels, label: useTranslateGeneral("this_year"), backgroundColor: theme.colors.green[6], negativeBackgroundColor: theme.colors.green[7], },
                  { ...statistics.total.previous, labels: statistics.total.labels, label: useTranslateGeneral("last_year"), backgroundColor: theme.colors.orange[7], negativeBackgroundColor: theme.colors.orange[6], },
                ]}
              />
            </Grid.Col>
            <Grid.Col md={4} >
              <TransactionProfitChart
                title={translateBase("stats_cards.today_chart.title")}
                type="revenue"
                // Magenta to light magenta
                background="linear-gradient(195deg, #94051f, #a3003d)"
                labels={statistics.today.chart_profit.labels || []}
                precision={0}
                orderWithRevenues={[
                  { ...statistics.today, ...statistics.today.chart_profit, backgroundColor: theme.colors.red[7], negativeBackgroundColor: theme.colors.blue[7], },
                ]}
              />
            </Grid.Col>
            <Grid.Col md={4} >
              <TransactionProfitChart
                title={translateBase("stats_cards.last_days.title", { days: statistics.recent_days.days })}
                type="revenue"
                background="linear-gradient(195deg, #051394, #0072a3)"
                labels={statistics.recent_days.chart_profit.labels || []}
                orderWithRevenues={[
                  { ...statistics.recent_days, ...statistics.recent_days.chart_profit, backgroundColor: theme.colors.blue[6], negativeBackgroundColor: theme.colors.blue[7], },
                ]}
              />
            </Grid.Col>
          </Grid>
          <Grid mt={10}>
            <Grid.Col md={12} >
              <DataTable
                records={statistics.best_seller.categorys}
                // define columns
                columns={[
                  {
                    accessor: 'name',
                    title: translateBase("stats_cards.datagrid.columns.name"),
                  },
                  {
                    accessor: 'revenue',
                    title: translateBase("stats_cards.datagrid.columns.revenue"),
                  },
                  {
                    accessor: 'expense',
                    title: translateBase("stats_cards.datagrid.columns.expense"),
                  },
                  {
                    accessor: 'profit',
                    title: translateBase("stats_cards.datagrid.columns.profit"),
                    render: ({ profit }) => <Text color={profit > 0 ? "green" : "red"}>{profit}</Text>,
                  },
                  {
                    accessor: 'profit_margin',
                    title: translateBase("stats_cards.datagrid.columns.profit_margin"),
                    render: ({ profit_margin }) => `${(profit_margin * 100).toPrecision(2)}%`,
                  },
                ]} />
            </Grid.Col>
          </Grid>
        </>
      }
    </Container>
  );
}