import { Grid, Group, Text, useMantineTheme, Container } from "@mantine/core";
import { useTauriContext } from "../../contexts";
import { StatsWithIcon } from "../../components/stats/statsWithIcon.stats";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBox, faCalendarAlt, faDollarSign, faMoneyBill } from "@fortawesome/free-solid-svg-icons";
import { Trans } from "react-i18next";
import { useTranslatePage, useTranslateGeneral } from "../../hooks";
import { TransactionRevenueChart } from "../../components/stats/transactionRevenueChart.stats";

const ChartContext = ({ i18nKey, values }: { i18nKey: string, values: { [key: string]: number } }) => {

  return (
    <Group grow>
      <Text size="sm"  >
        <Trans
          i18nKey={i18nKey.startsWith("general") ? i18nKey : `pages.home.stats_cards.${i18nKey}`}
          values={values}
          components={{ italic: <Text component="span" size="sm" color="blue.3" /> }}
        />
      </Text>
    </Group>)
}

export default function HomePage() {
  const theme = useMantineTheme();
  const translateBase = (key: string, context?: { [key: string]: any }) => useTranslatePage(`home.${key}`, { ...context })
  const { statistics } = useTauriContext();
  return (
    <Container size={"100%"}>
      {statistics &&
        <>
          <Grid>
            <Grid.Col md={3} >
              <StatsWithIcon
                color="linear-gradient(195deg, rgb(102, 187, 106), rgb(67, 160, 71))"
                icon={<FontAwesomeIcon icon={faMoneyBill} size="2x" />}
                title={translateBase("stats_cards.total_revenue_title")}
                count={statistics.total.sales.revenue - statistics.total.buy.revenue}
                fotter={
                  <ChartContext
                    i18nKey={"total_sales"}
                    values={{ val: statistics.total.sales.quantity }}
                  />}
              />
            </Grid.Col>
            <Grid.Col md={3} >
              <StatsWithIcon
                color="linear-gradient(195deg, rgb(236, 64, 122), rgb(216, 27, 96))"
                icon={<FontAwesomeIcon icon={faCalendarAlt} size="2x" />}
                title={translateBase("stats_cards.today_revenue_title")}
                count={statistics.today.sales.revenue - statistics.today.buy.revenue}
                fotter={
                  <ChartContext
                    i18nKey={"average_order_revenue"}
                    values={{ val: statistics.today.sales.quantity }}
                  />}
              />
            </Grid.Col>
            <Grid.Col md={3} >
              <StatsWithIcon
                color="linear-gradient(195deg, #49a3f1, #1A73E8);"
                icon={<FontAwesomeIcon icon={faDollarSign} size="2x" />}
                title={translateBase("stats_cards.open_orders_title")}
                count={statistics.turnover}
                fotter={
                  <ChartContext
                    i18nKey={"general.total_revenue"}
                    values={{ val: statistics.turnover }}
                  />}
              />
            </Grid.Col>
            <Grid.Col md={3} >
              <StatsWithIcon
                color="linear-gradient(195deg, rgb(154 64 236), rgb(117 27 216))"
                icon={<FontAwesomeIcon icon={faBox} size="2x" />}
                title={translateBase("stats_cards.best_selling_product_title")}
                count={statistics.total.present.sales.best_sellers[0]?.quantity || 0}
                fotter={
                  <Text size="sm"  >
                    {statistics.total.present.sales.best_sellers[0]?.item_name || ""}
                  </Text>}
              />
            </Grid.Col>
          </Grid>
          <Grid mt={25}>
            <Grid.Col md={4} >
              <TransactionRevenueChart
                title={translateBase("stats_cards.last_days_title", { days: statistics.recent_days.days })}
                type="revenue"
                background="linear-gradient(195deg, #051394, #0072a3)"
                labels={statistics.recent_days.labels || []}
                orderWithRevenues={[
                  { ...statistics.recent_days, backgroundColor: theme.colors.blue[6], },
                ]}
              />
            </Grid.Col>
            <Grid.Col md={4} >
              <TransactionRevenueChart
                title={translateBase("stats_cards.total_revenue_title")}
                showDatasetLabels
                type="revenue"
                // Green to light green
                background="linear-gradient(195deg, #0a4e0a, #00a300)"
                labels={statistics.total.labels || []}
                orderWithRevenues={[
                  { ...statistics.total.present, label: useTranslateGeneral("this_year"), backgroundColor: theme.colors.blue[6], },
                  { ...statistics.total.previous, label: useTranslateGeneral("last_year"), backgroundColor: theme.colors.orange[6], },
                ]}
              />
            </Grid.Col>
            <Grid.Col md={4} >
              <TransactionRevenueChart
                title={translateBase("stats_cards.today_revenue_title")}
                type="revenue"
                // Magenta to light magenta
                background="linear-gradient(195deg, #94051f, #a3003d)"
                labels={statistics.today.labels || []}
                precision={0}
                orderWithRevenues={[
                  { ...statistics.today, backgroundColor: theme.colors.gray[5], },
                ]}
              />
            </Grid.Col>
          </Grid>
        </>
      }
    </Container>
  );
}