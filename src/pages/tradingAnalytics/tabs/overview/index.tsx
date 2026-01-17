import { Box, Container, getGradient, Select, useMantineTheme } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { BarCardChart } from "@components/BarCardChart";
import { useEffect, useState } from "react";
import { useTranslatePages } from "@hooks/useTranslate.hook";
interface OverviewPanelProps {}

export const OverviewPanel = ({}: OverviewPanelProps) => {
  const [chartData, setChartData] = useState<{
    labels: string[];
    datasets: any[];
  }>({
    labels: [],
    datasets: [],
  });
  const [selectedChart, setSelectedChart] = useState<string>("most_traded");
  const theme = useMantineTheme();
  const { data } = useQuery({
    queryKey: ["itemPrices"],
    queryFn: () => api.items.getItemPriceOverview(),
  });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabOverview = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.overview.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOverview(`fields.${key}`, { ...context }, i18Key);
  const useTranslateCharts = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOverview(`charts.${key}`, { ...context }, i18Key);

  // Set Chart Data
  useEffect(() => {
    if (!data) return;
    const chartData = data[selectedChart as keyof typeof data] as any;
    setChartData({
      labels: chartData.labels,
      datasets: [
        {
          label: "today.bar_chart.datasets.profit",
          data: chartData.values,
          backgroundColor: "rgba(255, 99, 132, 0.5)",
        },
      ],
    });
  }, [data, selectedChart]);

  return (
    <Container size={"100%"}>
      <BarCardChart
        horizontal
        title={useTranslateCharts(`${selectedChart}.title`)}
        labels={data?.[selectedChart as keyof typeof data]?.labels || []}
        chartStyle={{ background: getGradient({ deg: 180, from: theme.colors.gray[8], to: theme.colors.gray[9] }, theme), height: "250px" }}
        datasets={chartData?.datasets || []}
        tooltipCallback={{
          label: (tooltipItem) => {
            const value = tooltipItem.raw;
            return `${useTranslateFields(`chart.${selectedChart}`)}: ${value}`;
          },
        }}
        context={
          <Box>
            <Select
              allowDeselect={false}
              value={selectedChart}
              data={[
                { value: "most_traded", label: useTranslateFields("chart.most_traded") },
                { value: "profit_margin", label: useTranslateFields("chart.profit_margin") },
                { value: "return_on_investment", label: useTranslateFields("chart.return_on_investment") },
              ]}
              onChange={(e) => {
                if (!e) return;
                setSelectedChart(e);
              }}
              placeholder={useTranslateFields("chart.placeholder")}
              label={useTranslateFields("chart.label")}
            />
          </Box>
        }
      />
      <Box w={"100%"}>
        <BarCardChart
          title={useTranslateCharts(`supply_and_demand.title`)}
          labels={data?.supply_and_demand.labels || []}
          chartStyle={{ background: getGradient({ deg: 180, from: theme.colors.gray[8], to: theme.colors.gray[9] }, theme), height: "300px" }}
          datasets={[
            {
              label: useTranslateCharts(`supply_and_demand.datasets.supply`),
              data: data?.supply_and_demand?.supply || [],
              backgroundColor: "rgb(0, 158, 33)",
            },
            {
              label: useTranslateCharts(`supply_and_demand.datasets.demand`),
              data: data?.supply_and_demand?.demand || [],
              backgroundColor: "rgb(242, 168, 60)",
            },
          ]}
        />
      </Box>
    </Container>
  );
};
