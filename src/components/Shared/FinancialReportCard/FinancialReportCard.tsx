import { Card, Grid, Stack, Text } from "@mantine/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { TauriTypes } from "$types";
import {
  faArrowTrendUp,
  faArrowTrendDown,
  faCoins,
  faChartLine,
  faShoppingCart,
  faReceipt,
  faCalculator,
  faPercent,
} from "@fortawesome/free-solid-svg-icons";
import { MetricCard } from "../MetricCard";

export interface FinancialReportCardProps {
  data: TauriTypes.FinancialReport | null;
  loading?: boolean;
}

export const FinancialReportCard = ({ data, loading = false }: FinancialReportCardProps) => {
  if (loading) {
    return (
      <Card p="lg" withBorder>
        <Text ta="center" c="dimmed">
          Loading financial report...
        </Text>
      </Card>
    );
  }

  if (!data) {
    return (
      <Card p="lg" withBorder>
        <Text ta="center" c="dimmed">
          No financial data available
        </Text>
      </Card>
    );
  }

  const profitTrend = data.total_profit > 0 ? "up" : data.total_profit < 0 ? "down" : "neutral";
  const roiTrend = data.roi > 0 ? "up" : data.roi < 0 ? "down" : "neutral";

  return (
    <Stack gap="sm">
      {/* Key Metrics Row - Most Important */}
      <Grid gutter={{ base: "xs", sm: "md" }}>
        <Grid.Col span={{ base: 12, xs: 6, sm: 6, md: 3, lg: 3 }}>
          <MetricCard
            icon={<FontAwesomeIcon icon={faCoins} />}
            title="Total Profit"
            value={data.total_profit}
            color="green"
            isCurrency
            trend={profitTrend}
          />
        </Grid.Col>

        <Grid.Col span={{ base: 12, xs: 6, sm: 6, md: 3, lg: 3 }}>
          <MetricCard icon={<FontAwesomeIcon icon={faPercent} />} title="ROI" value={data.roi} isPercentage trend={roiTrend} />
        </Grid.Col>

        <Grid.Col span={{ base: 12, xs: 6, sm: 6, md: 3, lg: 3 }}>
          <MetricCard
            icon={<FontAwesomeIcon icon={faCalculator} />}
            title="Profit Margin"
            value={data.profit_margin}
            color="var(--qf-profit)"
            isPercentage
          />
        </Grid.Col>

        <Grid.Col span={{ base: 12, xs: 6, sm: 6, md: 3, lg: 3 }}>
          <MetricCard icon={<FontAwesomeIcon icon={faReceipt} />} title="Total Transactions" value={data.total_transactions} color="orange" />
        </Grid.Col>
      </Grid>

      {/* Revenue & Sales */}
      <Grid gutter={{ base: "xs", sm: "md" }}>
        <Grid.Col span={{ base: 12, sm: 6, md: 4, lg: 4 }}>
          <MetricCard
            icon={<FontAwesomeIcon icon={faArrowTrendUp} />}
            title="Total Revenue"
            value={data.revenue}
            color="var(--qf-positive-color)"
            isCurrency
          />
        </Grid.Col>

        <Grid.Col span={{ base: 12, sm: 6, md: 4, lg: 4 }}>
          <MetricCard icon={<FontAwesomeIcon icon={faShoppingCart} />} title="Sales Count" value={data.sale_count} color="var(--qf-positive-color)" />
        </Grid.Col>

        <Grid.Col span={{ base: 12, sm: 12, md: 4, lg: 4 }}>
          <MetricCard
            icon={<FontAwesomeIcon icon={faCalculator} />}
            title="Average Revenue"
            value={data.average_revenue}
            color="var(--qf-positive-color)"
            isCurrency
          />
        </Grid.Col>
      </Grid>

      {/* Expenses & Purchases */}
      <Grid gutter={{ base: "xs", sm: "md" }}>
        <Grid.Col span={{ base: 12, sm: 6, md: 4, lg: 4 }}>
          <MetricCard
            icon={<FontAwesomeIcon icon={faArrowTrendDown} />}
            title="Total Expenses"
            value={data.expenses}
            color="var(--qf-negative-color)"
            isCurrency
          />
        </Grid.Col>

        <Grid.Col span={{ base: 12, sm: 6, md: 4, lg: 4 }}>
          <MetricCard
            icon={<FontAwesomeIcon icon={faShoppingCart} />}
            title="Purchases Count"
            value={data.purchases_count}
            color="var(--qf-negative-color)"
          />
        </Grid.Col>

        <Grid.Col span={{ base: 12, sm: 12, md: 4, lg: 4 }}>
          <MetricCard
            icon={<FontAwesomeIcon icon={faCalculator} />}
            title="Average Expense"
            value={data.average_expense}
            color="var(--qf-negative-color)"
            isCurrency
          />
        </Grid.Col>
      </Grid>

      {/* Averages */}
      <Grid gutter={{ base: "xs", sm: "md" }}>
        <Grid.Col span={{ base: 12, sm: 6, md: 6, lg: 6 }}>
          <MetricCard
            icon={<FontAwesomeIcon icon={faChartLine} />}
            title="Average Transaction"
            value={data.average_transaction}
            color="blue"
            isCurrency
          />
        </Grid.Col>

        <Grid.Col span={{ base: 12, sm: 6, md: 6, lg: 6 }}>
          <MetricCard
            icon={<FontAwesomeIcon icon={faCoins} />}
            title="Average Profit"
            value={data.average_profit}
            color="var(--qf-positive-color)"
            isCurrency
          />
        </Grid.Col>
      </Grid>
    </Stack>
  );
};
