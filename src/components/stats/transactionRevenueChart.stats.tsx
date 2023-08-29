import type { TooltipItem } from 'chart.js';
import { BarChart } from './barChart.stats';
import { StatisticTotalTransactionBuyAndSell, StatisticTransactionRevenueWithChart } from '$types/index';
import { useEffect, useState } from 'react';
import { useTranslateComponent, useTranslateGeneral } from '../../hooks';
import { Box, Group, Switch, Text } from '@mantine/core';
import { Trans } from 'react-i18next';

interface DataSet extends StatisticTotalTransactionBuyAndSell {
  label?: string,
  backgroundColor?: string;
}
interface TransactionRevenueChartProps {
  type: "revenue" | "quantity" | "shipping" | "average";
  title: string;
  background: string;
  precision?: number;
  labels: string[],
  // context?: React.ReactNode;
  orderWithRevenues: DataSet[],
  showDatasetLabels?: boolean;
}

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

export const TransactionRevenueChart = ({ type, showDatasetLabels, title, precision, background, orderWithRevenues, labels }: TransactionRevenueChartProps) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`transactionRevenueChart.${key}`, { ...context })
  const [datasets, setDatasets] = useState<{ label: string, data: number[], backgroundColor?: string; }[]>([]);
  const [tran_type, setTranType] = useState<"buy" | "sales">("sales");

  const [transactionRevenue, setTransactionRevenue] = useState<StatisticTransactionRevenueWithChart | undefined>(undefined);

  useEffect(() => {
    if (!orderWithRevenues) return;
    setDatasets(orderWithRevenues.map((orderWithRevenue) => {

      let revenue_chart = tran_type == "buy" ? orderWithRevenue?.buy.revenue_chart : orderWithRevenue?.sales.revenue_chart;
      let quantity_chart = tran_type == "buy" ? orderWithRevenue?.buy.quantity_chart : orderWithRevenue?.sales.quantity_chart;

      setTransactionRevenue(tran_type == "buy" ? orderWithRevenue?.buy : orderWithRevenue?.sales);
      return {
        label: orderWithRevenue?.label || "",
        data: type == "quantity" ? quantity_chart : revenue_chart,
        backgroundColor: orderWithRevenue?.backgroundColor || "",
      }
    }));
  }, [orderWithRevenues, tran_type]);

  return (
    <BarChart
      title={title}
      labels={labels}
      showDatasetLabels={showDatasetLabels}
      chartStyle={{ background: background, }}
      datasets={datasets}
      precision={precision}
      tooltipCallback={{
        title: function (items: TooltipItem<"bar">[]) {
          const item = items[0];
          if (!item) return "";
          if (item.dataset.label)
            return `${item.dataset.label} - ${item.label}`;
          else
            return item.label;
        },
        label: function () {
          return ""
        },
        afterLabel: function (context: TooltipItem<"bar">) {
          const labels: string[] = [];
          const dataSet = orderWithRevenues[context.datasetIndex];
          let revenue_chart = tran_type == "buy" ? dataSet?.buy.revenue_chart : dataSet?.sales.revenue_chart;
          let quantity_chart = tran_type == "buy" ? dataSet?.buy.quantity_chart : dataSet?.sales.quantity_chart;
          if (!dataSet) return "No data";
          labels.push(useTranslateSearch("revenue_label", { val: revenue_chart[context.dataIndex] }));
          labels.push(useTranslateSearch("quantity_label", { count: quantity_chart[context.dataIndex] }));
          return labels.join('\n');
        },
      }}
      context={
        <Group grow position="apart">
          <Box>
            <ChartContext
              i18nKey={"general.total_revenue"}
              values={{ val: transactionRevenue?.revenue || 0 }}
            />
            <ChartContext
              i18nKey={"general.total_quantity"}
              values={{ count: transactionRevenue?.quantity || 0 }}
            />
            <ChartContext
              i18nKey={"general.total_revenue_average"}
              values={{ count: transactionRevenue?.average || 0 }}
            />
          </Box>
          <Group position='right'>
            <Switch
              label={tran_type == "buy" ? useTranslateGeneral("buy_label") : useTranslateGeneral("sales_label")}
              checked={tran_type == "buy"}
              onChange={() => setTranType(tran_type == "buy" ? "sales" : "buy")}
            />
          </Group>
        </Group>
      }
    />
  );
}
