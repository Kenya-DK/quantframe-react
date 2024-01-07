import type { TooltipItem } from 'chart.js';
import { BarChart } from './barChart.stats';
import { ChartMultipleDto, StatisticProfitTransaction } from '$types/index';
import { useEffect, useState } from 'react';
import { Box, Group, Select } from '@mantine/core';
import { useTranslateComponent } from '../../hooks';
import { TextColor } from '../textColor';

interface DataSet extends ChartMultipleDto, StatisticProfitTransaction {
  label?: string,
  negativeBackgroundColor?: string;
  backgroundColor?: string;
}
interface TransactionProfitChartProps {
  type: "revenue" | "quantity" | "shipping" | "average";
  title: string;
  background: string;
  precision?: number;
  labels: string[],
  // context?: React.ReactNode;
  orderWithRevenues: DataSet[],
  showDatasetLabels?: boolean;
}

export const TransactionProfitChart = ({ showDatasetLabels, title, precision, background, orderWithRevenues, labels }: TransactionProfitChartProps) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`transactionRevenueChart.${key}`, { ...context }, i18Key)
  const useTranslateContext = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateSearch(`context.${key}`, { ...context }, i18Key)
  const [datasets, setDatasets] = useState<{ label: string, data: number[], backgroundColor?: any; }[]>([]);
  const modes = [
    { value: "0", label: useTranslateSearch("modes.sales.title",) },
    { value: "1", label: useTranslateSearch("modes.purchases.title") },
    { value: "2", label: useTranslateSearch("modes.quantity.title") },
    { value: "3", label: useTranslateSearch("modes.profit.title") },
  ]
  const [mode, setMode] = useState<string>("3");
  const [transactionRevenue, setTransactionRevenue] = useState<StatisticProfitTransaction | undefined>(undefined);

  useEffect(() => {
    if (!orderWithRevenues) return;
    setDatasets(orderWithRevenues.map((orderWithRevenue) => {
      let chart_values = orderWithRevenue?.values[Number(mode)];
      setTransactionRevenue(orderWithRevenues[0]);
      return {
        label: orderWithRevenue?.label || "",
        data: chart_values,
        // backgroundColor: orderWithRevenue?.backgroundColor || "",
        backgroundColor: (ctx2: any) => orderWithRevenue?.negativeBackgroundColor ? ctx2.parsed.y > 0 ? orderWithRevenue?.backgroundColor : orderWithRevenue.negativeBackgroundColor : orderWithRevenue?.backgroundColor || "",
      }
    }));
  }, [orderWithRevenues, mode]);

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
          for (let index = 0; index < modes.length; index++) {
            const element = modes[index];
            const vaule = dataSet?.values[index][context.dataIndex];
            labels.push(`${element.label}: ${vaule}`);
          }
          return labels.join('\n');
        },
      }}
      context={
        <>
          <Group grow position="apart" sx={{ position: "relative" }}>
            <Box>
              <TextColor i18nKey={useTranslateContext("profit", undefined, true)} values={{ val: transactionRevenue?.profit || 0 }} />
              <TextColor i18nKey={useTranslateContext("profit_margin", undefined, true)} values={{ val: ((transactionRevenue?.profit_margin || 0) * 100).toPrecision(2) }} />
            </Box>
            <Group position='right'>
              <Select
                maw={150}
                data={modes}
                placeholder="Select type"
                value={mode}
                onChange={(value) => {
                  if (!value) return;
                  setMode(value)
                }}
              />
            </Group>
          </Group>
          <Group >
            <TextColor i18nKey={useTranslateContext("footer", undefined, true)} values={{ sales: transactionRevenue?.sales || 0, purchases: transactionRevenue?.purchases || 0, trades: transactionRevenue?.number_of_trades || 0 }} />
          </Group>
        </>
      }
    />
  );
}

