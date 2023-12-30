import { Grid, Group, Text, useMantineTheme, Container, Image } from "@mantine/core";
import { StatsWithIcon } from "../../components/stats/statsWithIcon.stats";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCalendarAlt, faCubes, faMoneyBill } from "@fortawesome/free-solid-svg-icons";
import { Trans } from "react-i18next";
import { useTranslatePage, useTranslateGeneral } from "../../hooks";
import { TransactionRevenueChart } from "../../components/stats/transactionRevenueChart.stats";
import { useCacheContext, useWarframeMarketContextContext } from "../../contexts";
import { wfmThumbnail } from "@api/index";

const ChartContext = ({ i18nKey, values }: { i18nKey: string, values: { [key: string]: number | string } }) => {

  return (
    <Group grow>
      <Text size="sm"  >
        <Trans
          i18nKey={i18nKey.startsWith("general") ? i18nKey : `pages.home.stats_cards.${i18nKey}`}
          values={values}
          components={{
            italic: <Text component="span" size="sm" color="blue.3" />,
            qty: <FontAwesomeIcon icon={faCubes} />,
          }}
        />
      </Text>
    </Group>)
}

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
                count={statistics.total.sales.revenue - statistics.total.buy.revenue}
                fotter={
                  <ChartContext
                    i18nKey={"total.context"}
                    values={{
                      sales: statistics.total.sales.quantity,
                      buy: statistics.total.buy.quantity,
                      quantity: statistics.total.sales.quantity + statistics.total.buy.quantity,
                    }}
                  />}
              />
            </Grid.Col>
            <Grid.Col md={4} >
              <StatsWithIcon
                color="linear-gradient(195deg, rgb(236, 64, 122), rgb(216, 27, 96))"
                icon={<FontAwesomeIcon icon={faCalendarAlt} size="2x" />}
                title={translateBase("stats_cards.today.title")}
                count={statistics.today.sales.revenue - statistics.today.buy.revenue}
                fotter={
                  <ChartContext
                    i18nKey={"today.context"}
                    values={{
                      sales: statistics.today.sales.quantity,
                      buy: statistics.today.buy.quantity,
                      quantity: statistics.today.sales.quantity + statistics.today.buy.quantity,
                    }}
                  />}
              />
            </Grid.Col>
            <Grid.Col md={4} >
              <StatsWithIcon
                color="linear-gradient(195deg, rgb(154 64 236), rgb(117 27 216))"
                icon={<Image width={60} src={wfmThumbnail(images_map[statistics.total.present.popular_items.sell[0].url])} />}
                title={translateBase("stats_cards.best_selling.title")}
                count={statistics.total.present.popular_items.sell[0].turnover || 0}
                fotter={
                  <ChartContext
                    i18nKey={"best_selling.context"}
                    values={{
                      name: statistics.total.present.popular_items.sell[0].name || "",
                      sales: statistics.total.present.popular_items.sell[0].total_sold || 0,
                      buy: statistics.total.present.popular_items.sell[0].total_bought || 0,
                      quantity: statistics.total.present.popular_items.sell[0].quantity || 0,
                    }}
                  />}
              />
            </Grid.Col>
          </Grid>
          <Grid mt={25}>
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
          </Grid>
          {statistics.sales}
        </>
      }
    </Container>
  );
}