import { TauriTypes } from "$types";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { Box, Center, Text, useMantineTheme } from "@mantine/core";
import dayjs from "dayjs";
import { useMemo } from "react";
import { Chart } from "react-chartjs-2";

export type ChartViewProps = {
  transactions: TauriTypes.TransactionDto[];
  height?: number | string;
};

export function ChartView({ transactions, height = "65vh" }: ChartViewProps) {
  const theme = useMantineTheme();
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.tabs.transaction.chart.${key}`, { ...context }, i18Key);

  const { labels, revenue, expenses, cumulative } = useMemo(() => {
    const byDay = new Map<string, { revenue: number; expenses: number }>();
    for (const transaction of transactions) {
      const day = dayjs(transaction.created_at).format("YYYY-MM-DD");
      const entry = byDay.get(day) || { revenue: 0, expenses: 0 };

      if (transaction.transaction_type === TauriTypes.TransactionType.Sale) entry.revenue += transaction.price;
      else if (transaction.transaction_type === TauriTypes.TransactionType.Purchase) entry.expenses += transaction.price;

      byDay.set(day, entry);
    }

    const labels = Array.from(byDay.keys()).sort();
    const revenue = labels.map((day) => byDay.get(day)!.revenue);
    const expenses = labels.map((day) => byDay.get(day)!.expenses);

    let running = 0;
    const cumulative = labels.map((_, index) => (running += revenue[index] - expenses[index]));

    return { labels, revenue, expenses, cumulative };
  }, [transactions]);

  if (labels.length === 0) {
    return (
      <Center h={height}>
        <Text c="dimmed">{useTranslate("no_data")}</Text>
      </Center>
    );
  }

  const tickColor = theme.colors.gray[5];
  const gridColor = "rgba(255, 255, 255, 0.08)";

  return (
    <Box h={height} pos={"relative"}>
      <Chart
        type="bar"
        data={
          {
            labels,
            datasets: [
              {
                type: "bar",
                label: useTranslate("revenue"),
                data: revenue,
                backgroundColor: theme.colors.teal[6],
                borderRadius: 4,
                order: 3,
              },
              {
                type: "bar",
                label: useTranslate("expenses"),
                data: expenses,
                backgroundColor: theme.colors.red[7],
                borderRadius: 4,
                order: 2,
              },
              {
                type: "line",
                label: useTranslate("cumulative_profit"),
                data: cumulative,
                borderColor: theme.colors.blue[4],
                backgroundColor: theme.colors.blue[4],
                pointRadius: 2,
                tension: 0.3,
                yAxisID: "y1",
                order: 1,
              },
            ],
          } as any
        }
        options={
          {
            responsive: true,
            maintainAspectRatio: false,
            interaction: { mode: "index", intersect: false },
            plugins: {
              legend: { labels: { color: tickColor } },
              tooltip: {
                callbacks: {
                  label: (context: any) => `${context.dataset.label}: ${context.parsed.y.toLocaleString()}`,
                },
              },
            },
            scales: {
              x: {
                ticks: { color: tickColor },
                grid: { color: gridColor },
              },
              y: {
                position: "left",
                ticks: { color: tickColor },
                grid: { color: gridColor },
                title: { display: true, text: useTranslate("amount"), color: tickColor },
              },
              y1: {
                position: "right",
                ticks: { color: tickColor },
                grid: { drawOnChartArea: false },
                title: { display: true, text: useTranslate("cumulative_profit"), color: tickColor },
              },
            },
          } as any
        }
      />
    </Box>
  );
}
